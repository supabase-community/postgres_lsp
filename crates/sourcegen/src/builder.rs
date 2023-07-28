pub trait Builder {
    fn with<F>(&mut self, mut f: F) -> &mut Self
    where
        F: FnMut(&mut Self) -> &mut Self,
    {
        f(self)
    }

    fn finish(&mut self) -> String;
}
