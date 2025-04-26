

pub struct Event 
{
    callbacks: Vec<Box<dyn FnMut()>>,
}


impl Event 
{
    fn new() -> Self 
    {
        Self {callbacks: vec![]}
    }

    fn AddListener<F>(&mut self, f: F) where F : FnMut() + 'static,
    {
        self.callbacks.push(Box::new(f));
    }

    fn Invoke(&mut self) 
    {
        for callback in self.callbacks.iter_mut() 
        {
            callback();
        }
    }
}