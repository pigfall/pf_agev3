pub struct GPUProgram {
    id: Option<glow::Program>,
    // Force compiler to not implement Send and Sync, because OpenGL is not thread-safe.
    // thread_mark: PhantomData<*const u8>,
    // uniform_locations: RefCell<FxHashMap<ImmutableString, Option<UniformLocation>>>,
    //pub(crate) built_in_uniform_locations: [Option<UniformLocation>; BuiltInUniform::Count as usize],
}
