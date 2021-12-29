// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{ContainerNode, RenderNodeType};

define_render_node!(
    ContainerNode,
    ffi::GskContainerNode,
    RenderNodeType::ContainerNode
);
