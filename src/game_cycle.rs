use pancurses::Input;

pub trait GameCycle<T> {
    fn on_frame(&mut self, props: T) -> Result<(), ()>;
    fn on_input(&mut self, input: Input) -> ();
}
