use crate::graphics::*;
use std::sync::Arc;

pub trait Model<Vertex: super::Vertex, InstanceData: super::InstanceData>:
    Instance<InstanceData>
{
    fn mesh(&self) -> &Arc<Mesh<Vertex>>;
}
