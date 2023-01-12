use super::*;

pub enum NextScene {
    Push {
        scene: Box<dyn Scene>,
        transition: Option<Box<dyn Transition>>,
    },
    Swap {
        scene: Box<dyn Scene>,
        transition: Option<Box<dyn Transition>>,
    },
    PopSwap {
        scene: Box<dyn Scene>,
        transition: Option<Box<dyn Transition>>,
    },
    Pop {
        transition: Option<Box<dyn Transition>>,
    },
    // skips calling Scene::enter
    #[doc(hidden)]
    __InternalPush {
        scene: Box<dyn Scene>,
        transition: Option<Box<dyn Transition>>,
    },
    // skips calling Scene::enter
    #[doc(hidden)]
    __InternalSwap {
        scene: Box<dyn Scene>,
        transition: Option<Box<dyn Transition>>,
    },
    None,
}

impl NextScene {
    #[inline]
    pub fn new_push(scene: impl Scene + 'static) -> Self {
        NextScene::Push {
            scene: Box::new(scene),
            transition: None,
        }
    }

    #[inline]
    pub fn new_swap(scene: impl Scene + 'static) -> Self {
        NextScene::Swap {
            scene: Box::new(scene),
            transition: None,
        }
    }

    #[inline]
    pub fn new_pop_swap(scene: impl Scene + 'static) -> Self {
        NextScene::PopSwap {
            scene: Box::new(scene),
            transition: None,
        }
    }

    #[inline]
    pub fn new_pop() -> Self {
        NextScene::Pop { transition: None }
    }

    pub fn with_transition(mut self, transition: impl Transition + 'static) -> Self {
        match &mut self {
            NextScene::Push {
                transition: set_transition,
                ..
            }
            | NextScene::Swap {
                transition: set_transition,
                ..
            }
            | NextScene::PopSwap {
                transition: set_transition,
                ..
            }
            | NextScene::Pop {
                transition: set_transition,
            }
            | NextScene::__InternalPush {
                transition: set_transition,
                ..
            }
            | NextScene::__InternalSwap {
                transition: set_transition,
                ..
            } => {
                *set_transition = Some(Box::new(transition));
            }
            NextScene::None => {}
        }

        self
    }

    pub fn is_none(&self) -> bool {
        matches!(self, NextScene::None)
    }

    pub fn is_some(&self) -> bool {
        !matches!(self, NextScene::None)
    }

    pub fn take(&mut self) -> NextScene {
        let mut next_scene = NextScene::None;
        std::mem::swap(&mut next_scene, self);
        next_scene
    }
}

impl Default for NextScene {
    fn default() -> Self {
        Self::None
    }
}
