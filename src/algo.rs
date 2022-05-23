use super::*;
use crate::grid::{Position, Size, VECTOR_SIZE};
use bevy::prelude::Plugin as BevyPlugin;

// the global vector to be sorted
#[derive(Default)]
struct SortVec(Vec<usize>);

// an unique id
// this is used instead of entity.id() because, the e.id() is not reliable
// this value is also the height/value of the column
#[derive(Component)]
struct Number(usize);

// create ordered vec, shuffle, display
fn setup(mut commands: Commands, mut vec: ResMut<SortVec>) {
    for (x, height) in vec.0.iter().enumerate() {
        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.4, 0.6, 0.8),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Position { x: x as i32, y: 0 })
            .insert(Size::rect(1.0, (*height * 2) as f32))
            .insert(Number(x + 1));
    }

    vec.0.shuffle(&mut thread_rng());
}

// re renders the vector, only when the vector is changed
fn update(
    ents: Query<(Entity, &Number)>,
    mut q: Query<(&mut Position, &mut Sprite)>,
    vec: Res<SortVec>,
    sel: Res<SelectionSort>,
) {
    if vec.is_changed() {
        for (i, height) in vec.0.iter().enumerate() {
            for (e, id) in ents.iter() {
                if id.0 == *height {
                    match q.get_mut(e).ok() {
                        Some(t) => {
                            let (mut pos, mut spr) = t;

                            // set the position to the new index
                            pos.x = (i) as i32;

                            if i == sel.smallest {
                                spr.color = Color::rgb(0.0, 0.0, 0.0);
                            } else if i == sel.j {
                                spr.color = Color::rgb(1.0, 0.0, 0.0);
                            } else if i < sel.i {
                                spr.color = Color::rgb(0.0, 1.0, 0.0);
                            } else {
                                spr.color = Color::rgb(0.4, 0.6, 0.8);
                            }
                        }
                        None => println!("error ----: {}", e.id()),
                    }
                    // no need to loop through the rest of the vector
                    break;
                }
            }
        }
    }
}

pub trait Algorithm {
    fn sort(&mut self, vec: &mut Vec<usize>);
    fn reset(&mut self);
}

#[derive(Default)]
pub struct SelectionSort {
    i: usize,
    j: usize,
    smallest: usize,
}

impl SelectionSort {
    pub fn new() -> Self {
        SelectionSort {
            i: 0,
            j: 0,
            smallest: 0,
        }
    }
}

impl Algorithm for SelectionSort {
    fn sort(&mut self, vec: &mut Vec<usize>) {
        if self.j < VECTOR_SIZE as usize {
            if vec[self.j] < vec[self.smallest] {
                self.smallest = self.j;
            }
            self.j += 1;
        } else if self.i < VECTOR_SIZE as usize {
            vec.swap(self.i, self.smallest);
            self.i += 1;
            self.j = self.i;
            self.smallest = self.i;
        }
    }

    fn reset(&mut self) {
        self.i = 0;
        self.j = 0;
        self.smallest = 0;
    }
}

fn next(key: Res<Input<KeyCode>>, mut method: ResMut<SelectionSort>, mut vec: ResMut<SortVec>) {
    if key.pressed(KeyCode::Space) {
        for _i in 0..500 {
            method.sort(&mut vec.0);
        }
    }
    if key.just_pressed(KeyCode::Return) {
        for _i in 0..1 {
            method.sort(&mut vec.0);
        }
    }
}

pub struct Plugin;
impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SortVec((1..=VECTOR_SIZE).collect()))
            .insert_resource(SelectionSort::new())
            .add_startup_system(setup)
            .add_system(next.before(update))
            .add_system(update.after(next));
    }
}
