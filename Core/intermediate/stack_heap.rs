pub fn run(){
    // Stack: stores values in a last in first out format
    // Data on the stack must have a defined fixed size

    // Heap: stores values in a non-sequential order, can be resized dynamically

    struct TreeNode<T>{
        pub lef: Option<Box<TreeNode<T>>>,
        pub right: Option<Box<TreeNode<T>>>,
        pub key: T,

    }

    impl<T> TreeNode<T> {
        pub fn new(key: T) -> Self{
            TreeNode{
                lef: None,
                right: None,
                key,
            }
        }

        pub fn left(mut self, node: TreeNode<T>) -> Self{
            self.lef = Some(Box::new(node));
            self
        }

        pub fn right(mut self, node: TreeNode<T>) -> Self{
            self.lef = Some(Box::new(node));
            self
        }
    }


    let node1 = TreeNode::new(1);
    let node2 = TreeNode::new(2);
    let node3 = TreeNode::new(3);

    // let node1 = node1.left(node2).right(node3);

    // println!("{:?}", node1);





}