// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{GLShaderNode, RenderNodeType};

define_render_node!(
    GLShaderNode,
    ffi::GskGLShaderNode,
    RenderNodeType::GlShaderNode
);
