#![allow(dead_code, non_upper_case_globals, unused_variables)]
#![allow(non_camel_case_types, non_snake_case, missing_copy_implementations)]

mod __gl_imports {
    pub use std::mem;
    pub use std::os::raw;
}
    

// Common types from OpenGL 1.1
pub type GLenum = u32;
pub type GLboolean = u8;
pub type GLbitfield = u32;
pub type GLvoid = __gl_imports::raw::c_void;
pub type GLbyte = i8;
pub type GLshort = i16;
pub type GLint = i32;
pub type GLclampx = i32;
pub type GLubyte = u8;
pub type GLushort = u16;
pub type GLuint = u32;
pub type GLsizei = i32;
pub type GLfloat = f32;
pub type GLclampf = f32;
pub type GLdouble = f64;
pub type GLclampd = f64;
pub type GLeglImageOES = *const __gl_imports::raw::c_void;
pub type GLchar = i8;
pub type GLcharARB = i8;

pub type GLhandleARB = u32;

pub type GLhalfARB = u16;
pub type GLhalf = u16;

// Must be 32 bits
pub type GLfixed = GLint;

pub type GLintptr = isize;
pub type GLsizeiptr = isize;
pub type GLint64 = i64;
pub type GLuint64 = u64;
pub type GLintptrARB = isize;
pub type GLsizeiptrARB = isize;
pub type GLint64EXT = i64;
pub type GLuint64EXT = u64;

pub enum __GLsync {}
pub type GLsync = *const __GLsync;

// compatible with OpenCL cl_context
pub enum _cl_context {}
pub enum _cl_event {}

pub type GLDEBUGPROC = extern "system" fn(source: GLenum,
                                          gltype: GLenum,
                                          id: GLuint,
                                          severity: GLenum,
                                          length: GLsizei,
                                          message: *const GLchar,
                                          userParam: *mut GLvoid);
pub type GLDEBUGPROCARB = extern "system" fn(source: GLenum,
                                             gltype: GLenum,
                                             id: GLuint,
                                             severity: GLenum,
                                             length: GLsizei,
                                             message: *const GLchar,
                                             userParam: *mut GLvoid);
pub type GLDEBUGPROCKHR = extern "system" fn(source: GLenum,
                                             gltype: GLenum,
                                             id: GLuint,
                                             severity: GLenum,
                                             length: GLsizei,
                                             message: *const GLchar,
                                             userParam: *mut GLvoid);

// GLES 1 types
// "pub type GLclampx = i32;",

// GLES 1/2 types (tagged for GLES 1)
// "pub type GLbyte = i8;",
// "pub type GLubyte = u8;",
// "pub type GLfloat = GLfloat;",
// "pub type GLclampf = GLfloat;",
// "pub type GLfixed = i32;",
// "pub type GLint64 = i64;",
// "pub type GLuint64 = u64;",
// "pub type GLintptr = intptr_t;",
// "pub type GLsizeiptr = ssize_t;",

// GLES 1/2 types (tagged for GLES 2 - attribute syntax is limited)
// "pub type GLbyte = i8;",
// "pub type GLubyte = u8;",
// "pub type GLfloat = GLfloat;",
// "pub type GLclampf = GLfloat;",
// "pub type GLfixed = i32;",
// "pub type GLint64 = i64;",
// "pub type GLuint64 = u64;",
// "pub type GLint64EXT = i64;",
// "pub type GLuint64EXT = u64;",
// "pub type GLintptr = intptr_t;",
// "pub type GLsizeiptr = ssize_t;",

// GLES 2 types (none currently)

// Vendor extension types
pub type GLDEBUGPROCAMD = extern "system" fn(id: GLuint,
                                             category: GLenum,
                                             severity: GLenum,
                                             length: GLsizei,
                                             message: *const GLchar,
                                             userParam: *mut GLvoid);
pub type GLhalfNV = u16;
pub type GLvdpauSurfaceNV = GLintptr;

    
pub const ACTIVE_ATTRIBUTES: GLenum = 0x8B89;
pub const ACTIVE_ATTRIBUTE_MAX_LENGTH: GLenum = 0x8B8A;
pub const ACTIVE_TEXTURE: GLenum = 0x84E0;
pub const ACTIVE_UNIFORMS: GLenum = 0x8B86;
pub const ACTIVE_UNIFORM_MAX_LENGTH: GLenum = 0x8B87;
pub const ALIASED_LINE_WIDTH_RANGE: GLenum = 0x846E;
pub const ALIASED_POINT_SIZE_RANGE: GLenum = 0x846D;
pub const ALPHA: GLenum = 0x1906;
pub const ALPHA_BITS: GLenum = 0x0D55;
pub const ALWAYS: GLenum = 0x0207;
pub const ARRAY_BUFFER: GLenum = 0x8892;
pub const ARRAY_BUFFER_BINDING: GLenum = 0x8894;
pub const ATTACHED_SHADERS: GLenum = 0x8B85;
pub const BACK: GLenum = 0x0405;
pub const BLEND: GLenum = 0x0BE2;
pub const BLEND_COLOR: GLenum = 0x8005;
pub const BLEND_DST_ALPHA: GLenum = 0x80CA;
pub const BLEND_DST_RGB: GLenum = 0x80C8;
pub const BLEND_EQUATION: GLenum = 0x8009;
pub const BLEND_EQUATION_ALPHA: GLenum = 0x883D;
pub const BLEND_EQUATION_RGB: GLenum = 0x8009;
pub const BLEND_SRC_ALPHA: GLenum = 0x80CB;
pub const BLEND_SRC_RGB: GLenum = 0x80C9;
pub const BLUE_BITS: GLenum = 0x0D54;
pub const BOOL: GLenum = 0x8B56;
pub const BOOL_VEC2: GLenum = 0x8B57;
pub const BOOL_VEC3: GLenum = 0x8B58;
pub const BOOL_VEC4: GLenum = 0x8B59;
pub const BUFFER_SIZE: GLenum = 0x8764;
pub const BUFFER_USAGE: GLenum = 0x8765;
pub const BYTE: GLenum = 0x1400;
pub const CCW: GLenum = 0x0901;
pub const CLAMP_TO_EDGE: GLenum = 0x812F;
pub const COLOR_ATTACHMENT0: GLenum = 0x8CE0;
pub const COLOR_BUFFER_BIT: GLenum = 0x00004000;
pub const COLOR_CLEAR_VALUE: GLenum = 0x0C22;
pub const COLOR_WRITEMASK: GLenum = 0x0C23;
pub const COMPILE_STATUS: GLenum = 0x8B81;
pub const COMPRESSED_TEXTURE_FORMATS: GLenum = 0x86A3;
pub const CONSTANT_ALPHA: GLenum = 0x8003;
pub const CONSTANT_COLOR: GLenum = 0x8001;
pub const CULL_FACE: GLenum = 0x0B44;
pub const CULL_FACE_MODE: GLenum = 0x0B45;
pub const CURRENT_PROGRAM: GLenum = 0x8B8D;
pub const CURRENT_VERTEX_ATTRIB: GLenum = 0x8626;
pub const CW: GLenum = 0x0900;
pub const DECR: GLenum = 0x1E03;
pub const DECR_WRAP: GLenum = 0x8508;
pub const DELETE_STATUS: GLenum = 0x8B80;
pub const DEPTH_ATTACHMENT: GLenum = 0x8D00;
pub const DEPTH_BITS: GLenum = 0x0D56;
pub const DEPTH_BUFFER_BIT: GLenum = 0x00000100;
pub const DEPTH_CLEAR_VALUE: GLenum = 0x0B73;
pub const DEPTH_COMPONENT: GLenum = 0x1902;
pub const DEPTH_COMPONENT16: GLenum = 0x81A5;
pub const DEPTH_FUNC: GLenum = 0x0B74;
pub const DEPTH_RANGE: GLenum = 0x0B70;
pub const DEPTH_TEST: GLenum = 0x0B71;
pub const DEPTH_WRITEMASK: GLenum = 0x0B72;
pub const DITHER: GLenum = 0x0BD0;
pub const DONT_CARE: GLenum = 0x1100;
pub const DST_ALPHA: GLenum = 0x0304;
pub const DST_COLOR: GLenum = 0x0306;
pub const DYNAMIC_DRAW: GLenum = 0x88E8;
pub const ELEMENT_ARRAY_BUFFER: GLenum = 0x8893;
pub const ELEMENT_ARRAY_BUFFER_BINDING: GLenum = 0x8895;
pub const EQUAL: GLenum = 0x0202;
pub const EXTENSIONS: GLenum = 0x1F03;
pub const FALSE: GLboolean = 0;
pub const FASTEST: GLenum = 0x1101;
pub const FIXED: GLenum = 0x140C;
pub const FLOAT: GLenum = 0x1406;
pub const FLOAT_MAT2: GLenum = 0x8B5A;
pub const FLOAT_MAT3: GLenum = 0x8B5B;
pub const FLOAT_MAT4: GLenum = 0x8B5C;
pub const FLOAT_VEC2: GLenum = 0x8B50;
pub const FLOAT_VEC3: GLenum = 0x8B51;
pub const FLOAT_VEC4: GLenum = 0x8B52;
pub const FRAGMENT_SHADER: GLenum = 0x8B30;
pub const FRAMEBUFFER: GLenum = 0x8D40;
pub const FRAMEBUFFER_ATTACHMENT_OBJECT_NAME: GLenum = 0x8CD1;
pub const FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE: GLenum = 0x8CD0;
pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE: GLenum = 0x8CD3;
pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL: GLenum = 0x8CD2;
pub const FRAMEBUFFER_BINDING: GLenum = 0x8CA6;
pub const FRAMEBUFFER_COMPLETE: GLenum = 0x8CD5;
pub const FRAMEBUFFER_INCOMPLETE_ATTACHMENT: GLenum = 0x8CD6;
pub const FRAMEBUFFER_INCOMPLETE_DIMENSIONS: GLenum = 0x8CD9;
pub const FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT: GLenum = 0x8CD7;
pub const FRAMEBUFFER_UNSUPPORTED: GLenum = 0x8CDD;
pub const FRONT: GLenum = 0x0404;
pub const FRONT_AND_BACK: GLenum = 0x0408;
pub const FRONT_FACE: GLenum = 0x0B46;
pub const FUNC_ADD: GLenum = 0x8006;
pub const FUNC_REVERSE_SUBTRACT: GLenum = 0x800B;
pub const FUNC_SUBTRACT: GLenum = 0x800A;
pub const GENERATE_MIPMAP_HINT: GLenum = 0x8192;
pub const GEQUAL: GLenum = 0x0206;
pub const GREATER: GLenum = 0x0204;
pub const GREEN_BITS: GLenum = 0x0D53;
pub const HIGH_FLOAT: GLenum = 0x8DF2;
pub const HIGH_INT: GLenum = 0x8DF5;
pub const IMPLEMENTATION_COLOR_READ_FORMAT: GLenum = 0x8B9B;
pub const IMPLEMENTATION_COLOR_READ_TYPE: GLenum = 0x8B9A;
pub const INCR: GLenum = 0x1E02;
pub const INCR_WRAP: GLenum = 0x8507;
pub const INFO_LOG_LENGTH: GLenum = 0x8B84;
pub const INT: GLenum = 0x1404;
pub const INT_VEC2: GLenum = 0x8B53;
pub const INT_VEC3: GLenum = 0x8B54;
pub const INT_VEC4: GLenum = 0x8B55;
pub const INVALID_ENUM: GLenum = 0x0500;
pub const INVALID_FRAMEBUFFER_OPERATION: GLenum = 0x0506;
pub const INVALID_OPERATION: GLenum = 0x0502;
pub const INVALID_VALUE: GLenum = 0x0501;
pub const INVERT: GLenum = 0x150A;
pub const KEEP: GLenum = 0x1E00;
pub const LEQUAL: GLenum = 0x0203;
pub const LESS: GLenum = 0x0201;
pub const LINEAR: GLenum = 0x2601;
pub const LINEAR_MIPMAP_LINEAR: GLenum = 0x2703;
pub const LINEAR_MIPMAP_NEAREST: GLenum = 0x2701;
pub const LINES: GLenum = 0x0001;
pub const LINE_LOOP: GLenum = 0x0002;
pub const LINE_STRIP: GLenum = 0x0003;
pub const LINE_WIDTH: GLenum = 0x0B21;
pub const LINK_STATUS: GLenum = 0x8B82;
pub const LOW_FLOAT: GLenum = 0x8DF0;
pub const LOW_INT: GLenum = 0x8DF3;
pub const LUMINANCE: GLenum = 0x1909;
pub const LUMINANCE_ALPHA: GLenum = 0x190A;
pub const MAX_COMBINED_TEXTURE_IMAGE_UNITS: GLenum = 0x8B4D;
pub const MAX_CUBE_MAP_TEXTURE_SIZE: GLenum = 0x851C;
pub const MAX_FRAGMENT_UNIFORM_VECTORS: GLenum = 0x8DFD;
pub const MAX_RENDERBUFFER_SIZE: GLenum = 0x84E8;
pub const MAX_TEXTURE_IMAGE_UNITS: GLenum = 0x8872;
pub const MAX_TEXTURE_SIZE: GLenum = 0x0D33;
pub const MAX_VARYING_VECTORS: GLenum = 0x8DFC;
pub const MAX_VERTEX_ATTRIBS: GLenum = 0x8869;
pub const MAX_VERTEX_TEXTURE_IMAGE_UNITS: GLenum = 0x8B4C;
pub const MAX_VERTEX_UNIFORM_VECTORS: GLenum = 0x8DFB;
pub const MAX_VIEWPORT_DIMS: GLenum = 0x0D3A;
pub const MEDIUM_FLOAT: GLenum = 0x8DF1;
pub const MEDIUM_INT: GLenum = 0x8DF4;
pub const MIRRORED_REPEAT: GLenum = 0x8370;
pub const NEAREST: GLenum = 0x2600;
pub const NEAREST_MIPMAP_LINEAR: GLenum = 0x2702;
pub const NEAREST_MIPMAP_NEAREST: GLenum = 0x2700;
pub const NEVER: GLenum = 0x0200;
pub const NICEST: GLenum = 0x1102;
pub const NONE: GLenum = 0;
pub const NOTEQUAL: GLenum = 0x0205;
pub const NO_ERROR: GLenum = 0;
pub const NUM_COMPRESSED_TEXTURE_FORMATS: GLenum = 0x86A2;
pub const NUM_SHADER_BINARY_FORMATS: GLenum = 0x8DF9;
pub const ONE: GLenum = 1;
pub const ONE_MINUS_CONSTANT_ALPHA: GLenum = 0x8004;
pub const ONE_MINUS_CONSTANT_COLOR: GLenum = 0x8002;
pub const ONE_MINUS_DST_ALPHA: GLenum = 0x0305;
pub const ONE_MINUS_DST_COLOR: GLenum = 0x0307;
pub const ONE_MINUS_SRC_ALPHA: GLenum = 0x0303;
pub const ONE_MINUS_SRC_COLOR: GLenum = 0x0301;
pub const OUT_OF_MEMORY: GLenum = 0x0505;
pub const PACK_ALIGNMENT: GLenum = 0x0D05;
pub const POINTS: GLenum = 0x0000;
pub const POLYGON_OFFSET_FACTOR: GLenum = 0x8038;
pub const POLYGON_OFFSET_FILL: GLenum = 0x8037;
pub const POLYGON_OFFSET_UNITS: GLenum = 0x2A00;
pub const RED_BITS: GLenum = 0x0D52;
pub const RENDERBUFFER: GLenum = 0x8D41;
pub const RENDERBUFFER_ALPHA_SIZE: GLenum = 0x8D53;
pub const RENDERBUFFER_BINDING: GLenum = 0x8CA7;
pub const RENDERBUFFER_BLUE_SIZE: GLenum = 0x8D52;
pub const RENDERBUFFER_DEPTH_SIZE: GLenum = 0x8D54;
pub const RENDERBUFFER_GREEN_SIZE: GLenum = 0x8D51;
pub const RENDERBUFFER_HEIGHT: GLenum = 0x8D43;
pub const RENDERBUFFER_INTERNAL_FORMAT: GLenum = 0x8D44;
pub const RENDERBUFFER_RED_SIZE: GLenum = 0x8D50;
pub const RENDERBUFFER_STENCIL_SIZE: GLenum = 0x8D55;
pub const RENDERBUFFER_WIDTH: GLenum = 0x8D42;
pub const RENDERER: GLenum = 0x1F01;
pub const REPEAT: GLenum = 0x2901;
pub const REPLACE: GLenum = 0x1E01;
pub const RGB: GLenum = 0x1907;
pub const RGB565: GLenum = 0x8D62;
pub const RGB5_A1: GLenum = 0x8057;
pub const RGBA: GLenum = 0x1908;
pub const RGBA4: GLenum = 0x8056;
pub const SAMPLER_2D: GLenum = 0x8B5E;
pub const SAMPLER_CUBE: GLenum = 0x8B60;
pub const SAMPLES: GLenum = 0x80A9;
pub const SAMPLE_ALPHA_TO_COVERAGE: GLenum = 0x809E;
pub const SAMPLE_BUFFERS: GLenum = 0x80A8;
pub const SAMPLE_COVERAGE: GLenum = 0x80A0;
pub const SAMPLE_COVERAGE_INVERT: GLenum = 0x80AB;
pub const SAMPLE_COVERAGE_VALUE: GLenum = 0x80AA;
pub const SCISSOR_BOX: GLenum = 0x0C10;
pub const SCISSOR_TEST: GLenum = 0x0C11;
pub const SHADER_BINARY_FORMATS: GLenum = 0x8DF8;
pub const SHADER_COMPILER: GLenum = 0x8DFA;
pub const SHADER_SOURCE_LENGTH: GLenum = 0x8B88;
pub const SHADER_TYPE: GLenum = 0x8B4F;
pub const SHADING_LANGUAGE_VERSION: GLenum = 0x8B8C;
pub const SHORT: GLenum = 0x1402;
pub const SRC_ALPHA: GLenum = 0x0302;
pub const SRC_ALPHA_SATURATE: GLenum = 0x0308;
pub const SRC_COLOR: GLenum = 0x0300;
pub const STATIC_DRAW: GLenum = 0x88E4;
pub const STENCIL_ATTACHMENT: GLenum = 0x8D20;
pub const STENCIL_BACK_FAIL: GLenum = 0x8801;
pub const STENCIL_BACK_FUNC: GLenum = 0x8800;
pub const STENCIL_BACK_PASS_DEPTH_FAIL: GLenum = 0x8802;
pub const STENCIL_BACK_PASS_DEPTH_PASS: GLenum = 0x8803;
pub const STENCIL_BACK_REF: GLenum = 0x8CA3;
pub const STENCIL_BACK_VALUE_MASK: GLenum = 0x8CA4;
pub const STENCIL_BACK_WRITEMASK: GLenum = 0x8CA5;
pub const STENCIL_BITS: GLenum = 0x0D57;
pub const STENCIL_BUFFER_BIT: GLenum = 0x00000400;
pub const STENCIL_CLEAR_VALUE: GLenum = 0x0B91;
pub const STENCIL_FAIL: GLenum = 0x0B94;
pub const STENCIL_FUNC: GLenum = 0x0B92;
pub const STENCIL_INDEX8: GLenum = 0x8D48;
pub const STENCIL_PASS_DEPTH_FAIL: GLenum = 0x0B95;
pub const STENCIL_PASS_DEPTH_PASS: GLenum = 0x0B96;
pub const STENCIL_REF: GLenum = 0x0B97;
pub const STENCIL_TEST: GLenum = 0x0B90;
pub const STENCIL_VALUE_MASK: GLenum = 0x0B93;
pub const STENCIL_WRITEMASK: GLenum = 0x0B98;
pub const STREAM_DRAW: GLenum = 0x88E0;
pub const SUBPIXEL_BITS: GLenum = 0x0D50;
pub const TEXTURE: GLenum = 0x1702;
pub const TEXTURE0: GLenum = 0x84C0;
pub const TEXTURE1: GLenum = 0x84C1;
pub const TEXTURE10: GLenum = 0x84CA;
pub const TEXTURE11: GLenum = 0x84CB;
pub const TEXTURE12: GLenum = 0x84CC;
pub const TEXTURE13: GLenum = 0x84CD;
pub const TEXTURE14: GLenum = 0x84CE;
pub const TEXTURE15: GLenum = 0x84CF;
pub const TEXTURE16: GLenum = 0x84D0;
pub const TEXTURE17: GLenum = 0x84D1;
pub const TEXTURE18: GLenum = 0x84D2;
pub const TEXTURE19: GLenum = 0x84D3;
pub const TEXTURE2: GLenum = 0x84C2;
pub const TEXTURE20: GLenum = 0x84D4;
pub const TEXTURE21: GLenum = 0x84D5;
pub const TEXTURE22: GLenum = 0x84D6;
pub const TEXTURE23: GLenum = 0x84D7;
pub const TEXTURE24: GLenum = 0x84D8;
pub const TEXTURE25: GLenum = 0x84D9;
pub const TEXTURE26: GLenum = 0x84DA;
pub const TEXTURE27: GLenum = 0x84DB;
pub const TEXTURE28: GLenum = 0x84DC;
pub const TEXTURE29: GLenum = 0x84DD;
pub const TEXTURE3: GLenum = 0x84C3;
pub const TEXTURE30: GLenum = 0x84DE;
pub const TEXTURE31: GLenum = 0x84DF;
pub const TEXTURE4: GLenum = 0x84C4;
pub const TEXTURE5: GLenum = 0x84C5;
pub const TEXTURE6: GLenum = 0x84C6;
pub const TEXTURE7: GLenum = 0x84C7;
pub const TEXTURE8: GLenum = 0x84C8;
pub const TEXTURE9: GLenum = 0x84C9;
pub const TEXTURE_2D: GLenum = 0x0DE1;
pub const TEXTURE_BINDING_2D: GLenum = 0x8069;
pub const TEXTURE_BINDING_CUBE_MAP: GLenum = 0x8514;
pub const TEXTURE_CUBE_MAP: GLenum = 0x8513;
pub const TEXTURE_CUBE_MAP_NEGATIVE_X: GLenum = 0x8516;
pub const TEXTURE_CUBE_MAP_NEGATIVE_Y: GLenum = 0x8518;
pub const TEXTURE_CUBE_MAP_NEGATIVE_Z: GLenum = 0x851A;
pub const TEXTURE_CUBE_MAP_POSITIVE_X: GLenum = 0x8515;
pub const TEXTURE_CUBE_MAP_POSITIVE_Y: GLenum = 0x8517;
pub const TEXTURE_CUBE_MAP_POSITIVE_Z: GLenum = 0x8519;
pub const TEXTURE_MAG_FILTER: GLenum = 0x2800;
pub const TEXTURE_MIN_FILTER: GLenum = 0x2801;
pub const TEXTURE_WRAP_S: GLenum = 0x2802;
pub const TEXTURE_WRAP_T: GLenum = 0x2803;
pub const TRIANGLES: GLenum = 0x0004;
pub const TRIANGLE_FAN: GLenum = 0x0006;
pub const TRIANGLE_STRIP: GLenum = 0x0005;
pub const TRUE: GLboolean = 1;
pub const UNPACK_ALIGNMENT: GLenum = 0x0CF5;
pub const UNSIGNED_BYTE: GLenum = 0x1401;
pub const UNSIGNED_INT: GLenum = 0x1405;
pub const UNSIGNED_SHORT: GLenum = 0x1403;
pub const UNSIGNED_SHORT_4_4_4_4: GLenum = 0x8033;
pub const UNSIGNED_SHORT_5_5_5_1: GLenum = 0x8034;
pub const UNSIGNED_SHORT_5_6_5: GLenum = 0x8363;
pub const VALIDATE_STATUS: GLenum = 0x8B83;
pub const VENDOR: GLenum = 0x1F00;
pub const VERSION: GLenum = 0x1F02;
pub const VERTEX_ATTRIB_ARRAY_BUFFER_BINDING: GLenum = 0x889F;
pub const VERTEX_ATTRIB_ARRAY_ENABLED: GLenum = 0x8622;
pub const VERTEX_ATTRIB_ARRAY_NORMALIZED: GLenum = 0x886A;
pub const VERTEX_ATTRIB_ARRAY_POINTER: GLenum = 0x8645;
pub const VERTEX_ATTRIB_ARRAY_SIZE: GLenum = 0x8623;
pub const VERTEX_ATTRIB_ARRAY_STRIDE: GLenum = 0x8624;
pub const VERTEX_ATTRIB_ARRAY_TYPE: GLenum = 0x8625;
pub const VERTEX_SHADER: GLenum = 0x8B31;
pub const VIEWPORT: GLenum = 0x0BA2;
pub const ZERO: GLenum = 0;

extern "system" {
#[link_name="glActiveTexture"]              pub fn ActiveTexture(texture: GLenum);
#[link_name="glAttachShader"]               pub fn AttachShader(program: u32, shader: u32);
#[link_name="glBindAttribLocation"]         pub fn BindAttribLocation(program: u32, index: u32, name: *const GLchar);
#[link_name="glBindBuffer"]                 pub fn BindBuffer(target: GLenum, buffer: u32);
#[link_name="glBindFramebuffer"]            pub fn BindFramebuffer(target: GLenum, framebuffer: u32);
#[link_name="glBindRenderbuffer"]           pub fn BindRenderbuffer(target: GLenum, renderbuffer: u32);
#[link_name="glBindTexture"]                pub fn BindTexture(target: GLenum, texture: u32);
#[link_name="glBlendColor"]                 pub fn BlendColor(red: f32, green: f32, blue: f32, alpha: f32);
#[link_name="glBlendEquation"]              pub fn BlendEquation(mode: GLenum);
#[link_name="glBlendEquationSeparate"]      pub fn BlendEquationSeparate(modeRGB: GLenum, modeAlpha: GLenum);
#[link_name="glBlendFunc"]                  pub fn BlendFunc(sfactor: GLenum, dfactor: GLenum);
#[link_name="glBlendFuncSeparate"]          pub fn BlendFuncSeparate(sfactorRGB: GLenum, dfactorRGB: GLenum, sfactorAlpha: GLenum, dfactorAlpha: GLenum);
#[link_name="glBufferData"]                 pub fn BufferData(target: GLenum, size: i32, data: *const GLvoid, usage: GLenum);
#[link_name="glBufferSubData"]              pub fn BufferSubData(target: GLenum, offset: i32, size: i32, data: *const GLvoid);
#[link_name="glCheckFramebufferStatus"]     pub fn CheckFramebufferStatus(target: GLenum) -> GLenum;
#[link_name="glClear"]                      pub fn Clear(mask: GLbitfield);
#[link_name="glClearColor"]                 pub fn ClearColor(red: f32, green: f32, blue: f32, alpha: f32);
#[link_name="glClearDepthf"]                pub fn ClearDepthf(d: f32);
#[link_name="glClearStencil"]               pub fn ClearStencil(s: i32);
#[link_name="glColorMask"]                  pub fn ColorMask(red: GLboolean, green: GLboolean, blue: GLboolean, alpha: GLboolean);
#[link_name="glCompileShader"]              pub fn CompileShader(shader: u32);
#[link_name="glCompressedTexImage2D"]       pub fn CompressedTexImage2D(target: GLenum, level: i32, internalformat: GLenum, width: i32, height: i32, border: i32, imageSize: i32, data: *const GLvoid);
#[link_name="glCompressedTexSubImage2D"]    pub fn CompressedTexSubImage2D(target: GLenum, level: i32, xoffset: i32, yoffset: i32, width: i32, height: i32, format: GLenum, imageSize: i32, data: *const GLvoid);
#[link_name="glCopyTexImage2D"]             pub fn CopyTexImage2D(target: GLenum, level: i32, internalformat: GLenum, x: i32, y: i32, width: i32, height: i32, border: i32);
#[link_name="glCopyTexSubImage2D"]          pub fn CopyTexSubImage2D(target: GLenum, level: i32, xoffset: i32, yoffset: i32, x: i32, y: i32, width: i32, height: i32);
#[link_name="glCreateProgram"]              pub fn CreateProgram() -> u32;
#[link_name="glCreateShader"]               pub fn CreateShader(type_: GLenum) -> u32;
#[link_name="glCullFace"]                   pub fn CullFace(mode: GLenum);
#[link_name="glDeleteBuffers"]              pub fn DeleteBuffers(n: i32, buffers: *const u32);
#[link_name="glDeleteFramebuffers"]         pub fn DeleteFramebuffers(n: i32, framebuffers: *const u32);
#[link_name="glDeleteProgram"]              pub fn DeleteProgram(program: u32);
#[link_name="glDeleteRenderbuffers"]        pub fn DeleteRenderbuffers(n: i32, renderbuffers: *const u32);
#[link_name="glDeleteShader"]               pub fn DeleteShader(shader: u32);
#[link_name="glDeleteTextures"]             pub fn DeleteTextures(n: i32, textures: *const u32);
#[link_name="glDepthFunc"]                  pub fn DepthFunc(func: GLenum);
#[link_name="glDepthMask"]                  pub fn DepthMask(flag: GLboolean);
#[link_name="glDepthRangef"]                pub fn DepthRangef(n: f32, f: f32);
#[link_name="glDetachShader"]               pub fn DetachShader(program: u32, shader: u32);
#[link_name="glDisable"]                    pub fn Disable(cap: GLenum);
#[link_name="glDisableVertexAttribArray"]   pub fn DisableVertexAttribArray(index: u32);
#[link_name="glDrawArrays"]                 pub fn DrawArrays(mode: GLenum, first: i32, count: i32);
#[link_name="glDrawElements"]               pub fn DrawElements(mode: GLenum, count: i32, type_: GLenum, indices: *const GLvoid);
#[link_name="glEnable"]                     pub fn Enable(cap: GLenum);
#[link_name="glEnableVertexAttribArray"]    pub fn EnableVertexAttribArray(index: u32);
#[link_name="glFinish"]                     pub fn Finish();
#[link_name="glFlush"]                      pub fn Flush();
#[link_name="glFramebufferRenderbuffer"]    pub fn FramebufferRenderbuffer(target: GLenum, attachment: GLenum, renderbuffertarget: GLenum, renderbuffer: u32);
#[link_name="glFramebufferTexture2D"]       pub fn FramebufferTexture2D(target: GLenum, attachment: GLenum, textarget: GLenum, texture: u32, level: i32);
#[link_name="glFrontFace"]                  pub fn FrontFace(mode: GLenum);
#[link_name="glGenBuffers"]                 pub fn GenBuffers(n: i32, buffers: *mut u32);
#[link_name="glGenFramebuffers"]            pub fn GenFramebuffers(n: i32, framebuffers: *mut u32);
#[link_name="glGenRenderbuffers"]           pub fn GenRenderbuffers(n: i32, renderbuffers: *mut u32);
#[link_name="glGenTextures"]                pub fn GenTextures(n: i32, textures: *mut u32);
#[link_name="glGenerateMipmap"]             pub fn GenerateMipmap(target: GLenum);
#[link_name="glGetActiveAttrib"]            pub fn GetActiveAttrib(program: u32, index: u32, bufSize: i32, length: *mut i32, size: *mut i32, type_: *mut GLenum, name: *mut GLchar);
#[link_name="glGetActiveUniform"]           pub fn GetActiveUniform(program: u32, index: u32, bufSize: i32, length: *mut i32, size: *mut i32, type_: *mut GLenum, name: *mut GLchar);
#[link_name="glGetAttachedShaders"]         pub fn GetAttachedShaders(program: u32, maxCount: i32, count: *mut i32, shaders: *mut u32);
#[link_name="glGetAttribLocation"]          pub fn GetAttribLocation(program: u32, name: *const GLchar) -> i32;
#[link_name="glGetBooleanv"]                pub fn GetBooleanv(pname: GLenum, data: *mut GLboolean);
#[link_name="glGetBufferParameteriv"]       pub fn GetBufferParameteriv(target: GLenum, pname: GLenum, params: *mut i32);
#[link_name="glGetError"]                   pub fn GetError() -> GLenum;
#[link_name="glGetFloatv"]                  pub fn GetFloatv(pname: GLenum, data: *mut f32);
#[link_name="glGetFramebufferAttachmentParameteriv"] pub fn GetFramebufferAttachmentParameteriv(target: GLenum, attachment: GLenum, pname: GLenum, params: *mut i32);
#[link_name="glGetIntegerv"]                pub fn GetIntegerv(pname: GLenum, data: *mut i32);
#[link_name="glGetProgramInfoLog"]          pub fn GetProgramInfoLog(program: u32, bufSize: i32, length: *mut i32, infoLog: *mut GLchar);
#[link_name="glGetProgramiv"]               pub fn GetProgramiv(program: u32, pname: GLenum, params: *mut i32);
#[link_name="glGetRenderbufferParameteriv"] pub fn GetRenderbufferParameteriv(target: GLenum, pname: GLenum, params: *mut i32);
#[link_name="glGetShaderInfoLog"]           pub fn GetShaderInfoLog(shader: u32, bufSize: i32, length: *mut i32, infoLog: *mut GLchar);
#[link_name="glGetShaderPrecisionFormat"]   pub fn GetShaderPrecisionFormat(shadertype: GLenum, precisiontype: GLenum, range: *mut i32, precision: *mut i32);
#[link_name="glGetShaderSource"]            pub fn GetShaderSource(shader: u32, bufSize: i32, length: *mut i32, source: *mut GLchar);
#[link_name="glGetShaderiv"]                pub fn GetShaderiv(shader: u32, pname: GLenum, params: *mut i32);
#[link_name="glGetString"]                  pub fn GetString(name: GLenum) -> *const GLubyte;
#[link_name="glGetTexParameterfv"]          pub fn GetTexParameterfv(target: GLenum, pname: GLenum, params: *mut f32);
#[link_name="glGetTexParameteriv"]          pub fn GetTexParameteriv(target: GLenum, pname: GLenum, params: *mut i32);
#[link_name="glGetUniformLocation"]         pub fn GetUniformLocation(program: u32, name: *const GLchar) -> i32;
#[link_name="glGetUniformfv"]               pub fn GetUniformfv(program: u32, location: i32, params: *mut f32);
#[link_name="glGetUniformiv"]               pub fn GetUniformiv(program: u32, location: i32, params: *mut i32);
#[link_name="glGetVertexAttribPointerv"]    pub fn GetVertexAttribPointerv(index: u32, pname: GLenum, pointer: *const *mut GLvoid);
#[link_name="glGetVertexAttribfv"]          pub fn GetVertexAttribfv(index: u32, pname: GLenum, params: *mut f32);
#[link_name="glGetVertexAttribiv"]          pub fn GetVertexAttribiv(index: u32, pname: GLenum, params: *mut i32);
#[link_name="glHint"]                       pub fn Hint(target: GLenum, mode: GLenum);
#[link_name="glIsBuffer"]                   pub fn IsBuffer(buffer: u32) -> GLboolean;
#[link_name="glIsEnabled"]                  pub fn IsEnabled(cap: GLenum) -> GLboolean;
#[link_name="glIsFramebuffer"]              pub fn IsFramebuffer(framebuffer: u32) -> GLboolean;
#[link_name="glIsProgram"]                  pub fn IsProgram(program: u32) -> GLboolean;
#[link_name="glIsRenderbuffer"]             pub fn IsRenderbuffer(renderbuffer: u32) -> GLboolean;
#[link_name="glIsShader"]                   pub fn IsShader(shader: u32) -> GLboolean;
#[link_name="glIsTexture"]                  pub fn IsTexture(texture: u32) -> GLboolean;
#[link_name="glLineWidth"]                  pub fn LineWidth(width: f32);
#[link_name="glLinkProgram"]                pub fn LinkProgram(program: u32);
#[link_name="glPixelStorei"]                pub fn PixelStorei(pname: GLenum, param: i32);
#[link_name="glPolygonOffset"]              pub fn PolygonOffset(factor: f32, units: f32);
#[link_name="glReadPixels"]                 pub fn ReadPixels(x: i32, y: i32, width: i32, height: i32, format: GLenum, type_: GLenum, pixels: *mut GLvoid);
#[link_name="glReleaseShaderCompiler"]      pub fn ReleaseShaderCompiler();
#[link_name="glRenderbufferStorage"]        pub fn RenderbufferStorage(target: GLenum, internalformat: GLenum, width: i32, height: i32);
#[link_name="glSampleCoverage"]             pub fn SampleCoverage(value: f32, invert: GLboolean);
#[link_name="glScissor"]                    pub fn Scissor(x: i32, y: i32, width: i32, height: i32);
#[link_name="glShaderBinary"]               pub fn ShaderBinary(count: i32, shaders: *const u32, binaryformat: GLenum, binary: *const GLvoid, length: i32);
#[link_name="glShaderSource"]               pub fn ShaderSource(shader: u32, count: i32, string: *const *const GLchar, length: *const i32);
#[link_name="glStencilFunc"]                pub fn StencilFunc(func: GLenum, ref_: i32, mask: u32);
#[link_name="glStencilFuncSeparate"]        pub fn StencilFuncSeparate(face: GLenum, func: GLenum, ref_: i32, mask: u32);
#[link_name="glStencilMask"]                pub fn StencilMask(mask: u32);
#[link_name="glStencilMaskSeparate"]        pub fn StencilMaskSeparate(face: GLenum, mask: u32);
#[link_name="glStencilOp"]                  pub fn StencilOp(fail: GLenum, zfail: GLenum, zpass: GLenum);
#[link_name="glStencilOpSeparate"]          pub fn StencilOpSeparate(face: GLenum, sfail: GLenum, dpfail: GLenum, dppass: GLenum);
#[link_name="glTexImage2D"]                 pub fn TexImage2D(target: GLenum, level: i32, internalformat: i32, width: i32, height: i32, border: i32, format: GLenum, type_: GLenum, pixels: *const GLvoid);
#[link_name="glTexParameterf"]              pub fn TexParameterf(target: GLenum, pname: GLenum, param: f32);
#[link_name="glTexParameterfv"]             pub fn TexParameterfv(target: GLenum, pname: GLenum, params: *const f32);
#[link_name="glTexParameteri"]              pub fn TexParameteri(target: GLenum, pname: GLenum, param: i32);
#[link_name="glTexParameteriv"]             pub fn TexParameteriv(target: GLenum, pname: GLenum, params: *const i32);
#[link_name="glTexSubImage2D"]              pub fn TexSubImage2D(target: GLenum, level: i32, xoffset: i32, yoffset: i32, width: i32, height: i32, format: GLenum, type_: GLenum, pixels: *const GLvoid);
#[link_name="glUniform1f"]                  pub fn Uniform1f(location: i32, v0: f32);
#[link_name="glUniform1fv"]                 pub fn Uniform1fv(location: i32, count: i32, value: *const f32);
#[link_name="glUniform1i"]                  pub fn Uniform1i(location: i32, v0: i32);
#[link_name="glUniform1iv"]                 pub fn Uniform1iv(location: i32, count: i32, value: *const i32);
#[link_name="glUniform2f"]                  pub fn Uniform2f(location: i32, v0: f32, v1: f32);
#[link_name="glUniform2fv"]                 pub fn Uniform2fv(location: i32, count: i32, value: *const f32);
#[link_name="glUniform2i"]                  pub fn Uniform2i(location: i32, v0: i32, v1: i32);
#[link_name="glUniform2iv"]                 pub fn Uniform2iv(location: i32, count: i32, value: *const i32);
#[link_name="glUniform3f"]                  pub fn Uniform3f(location: i32, v0: f32, v1: f32, v2: f32);
#[link_name="glUniform3fv"]                 pub fn Uniform3fv(location: i32, count: i32, value: *const f32);
#[link_name="glUniform3i"]                  pub fn Uniform3i(location: i32, v0: i32, v1: i32, v2: i32);
#[link_name="glUniform3iv"]                 pub fn Uniform3iv(location: i32, count: i32, value: *const i32);
#[link_name="glUniform4f"]                  pub fn Uniform4f(location: i32, v0: f32, v1: f32, v2: f32, v3: f32);
#[link_name="glUniform4fv"]                 pub fn Uniform4fv(location: i32, count: i32, value: *const f32);
#[link_name="glUniform4i"]                  pub fn Uniform4i(location: i32, v0: i32, v1: i32, v2: i32, v3: i32);
#[link_name="glUniform4iv"]                 pub fn Uniform4iv(location: i32, count: i32, value: *const i32);
#[link_name="glUniformMatrix2fv"]           pub fn UniformMatrix2fv(location: i32, count: i32, transpose: GLboolean, value: *const f32);
#[link_name="glUniformMatrix3fv"]           pub fn UniformMatrix3fv(location: i32, count: i32, transpose: GLboolean, value: *const f32);
#[link_name="glUniformMatrix4fv"]           pub fn UniformMatrix4fv(location: i32, count: i32, transpose: GLboolean, value: *const f32);
#[link_name="glUseProgram"]                 pub fn UseProgram(program: u32);
#[link_name="glValidateProgram"]            pub fn ValidateProgram(program: u32);
#[link_name="glVertexAttrib1f"]             pub fn VertexAttrib1f(index: u32, x: f32);
#[link_name="glVertexAttrib1fv"]            pub fn VertexAttrib1fv(index: u32, v: *const f32);
#[link_name="glVertexAttrib2f"]             pub fn VertexAttrib2f(index: u32, x: f32, y: f32);
#[link_name="glVertexAttrib2fv"]            pub fn VertexAttrib2fv(index: u32, v: *const f32);
#[link_name="glVertexAttrib3f"]             pub fn VertexAttrib3f(index: u32, x: f32, y: f32, z: f32);
#[link_name="glVertexAttrib3fv"]            pub fn VertexAttrib3fv(index: u32, v: *const f32);
#[link_name="glVertexAttrib4f"]             pub fn VertexAttrib4f(index: u32, x: f32, y: f32, z: f32, w: f32);
#[link_name="glVertexAttrib4fv"]            pub fn VertexAttrib4fv(index: u32, v: *const f32);
#[link_name="glVertexAttribPointer"]        pub fn VertexAttribPointer(index: u32, size: i32, type_: GLenum, normalized: GLboolean, stride: i32, pointer: *const GLvoid);
#[link_name="glViewport"]                   pub fn Viewport(x: i32, y: i32, width: i32, height: i32);
}
