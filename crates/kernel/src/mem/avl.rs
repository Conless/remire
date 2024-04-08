// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use core::{
    cmp::max,
    ptr::{null, null_mut},
};

pub struct AVLTree<T: PartialOrd + Copy>(*mut AVLTreeNode<T>);

/// AVL Tree Node
/// 
/// This struct is used to represent a node in AVL Tree. The `data` field may be replaced by the address of the node.
struct AVLTreeNode<T: PartialOrd + Copy> {
    data: T,
    height: usize,
    left: *mut AVLTreeNode<T>,
    right: *mut AVLTreeNode<T>,
}

impl<T: PartialOrd + Copy> AVLTreeNode<T> {
    fn init(&mut self, data: T) {
        self.data = data;
        self.height = 1;
        self.left = core::ptr::null_mut();
        self.right = core::ptr::null_mut();
    }
}

impl<T: PartialOrd + Copy> AVLTree<T> {
    /// Get the height of the current node
    fn get_height(node: *mut AVLTreeNode<T>) -> usize {
        if node.is_null() {
            return 0;
        }
        unsafe { (*node).height }
    }

    /// Perform a LL rotation
    /// 
    /// LL(Y rotates to the left):
    ///    k2                     k1
    ///   / \                    / \
    ///   X  k1       ======     k2  Z
    ///  / \                    / \
    /// Y  Z                   X   Y
    fn ll_rotate(mut node: *mut AVLTreeNode<T>) {
        unsafe {
            let new_root = (*node).left;
            (*node).left = (*new_root).right;
            (*new_root).right = node;
            (*node).height = max(
                Self::get_height((*node).left),
                Self::get_height((*node).right),
            ) + 1;
            (*new_root).height = max(
                Self::get_height((*new_root).left),
                Self::get_height((*new_root).right),
            ) + 1;
            node = new_root;
        }
    }

    /// Perform a RR rotation
    /// 
    /// RR(Y rotates to the right):
    ///        k2                 k1
    ///       / \                / \
    ///      k1  Z     ======   X  k2
    ///     / \                    / \
    ///    X   Y                  Y   Z
    fn rr_rotate(mut node: *mut AVLTreeNode<T>) {
        unsafe {
            let new_root = (*node).right;
            (*node).right = (*new_root).left;
            (*new_root).left = node;
            (*node).height = max(
                Self::get_height((*node).left),
                Self::get_height((*node).right),
            ) + 1;
            (*new_root).height = max(
                Self::get_height((*new_root).left),
                Self::get_height((*new_root).right),
            ) + 1;
            node = new_root;
        }
    }

    /// Perform a LR rotation
    /// 
    /// LR(B rotates to the left, then C rotates to the right):
    ///     k3                       k3                    k2
    ///     / \                      / \                   / \
    ///    k1  D                    k2  D                k1   k3
    ///   / \      ======          / \     ======        / \  / \
    ///  A   k2                   k1  C                 A  B C   D
    ///     / \                 / \
    ///    B  C                A   B
    fn lr_rotate(mut node: *mut AVLTreeNode<T>) {
        unsafe {
            Self::rr_rotate((*node).left);
            Self::ll_rotate(node);
        }
    }

    /// Perform a RL rotation
    /// 
    /// RL(D rotates to the right, then C rotates to the left):
    ///   k3                      k3                       k2
    ///  / \                     / \                      / \
    /// A  k1                   A   k2                   k3  k1 
    ///    / \      ======          / \        ======    / \ / \
    ///   k2  B                    C   k1               A  C D  B
    ///   / \                         / \
    ///  C   D                       D   B 
    fn rl_rotate(mut node: *mut AVLTreeNode<T>) {
        unsafe {
            Self::ll_rotate((*node).right);
            Self::rr_rotate(node);
        }
    }

    /// Find a node in the current subtree
    fn find_in_node(node: *mut AVLTreeNode<T>, data: T) -> bool {
        if node.is_null() {
            return false;
        }
        if data == unsafe { (*node).data } {
            return true;
        }
        if data < unsafe { (*node).data } {
            return Self::find_in_node(unsafe { (*node).left }, data);
        }
        Self::find_in_node(unsafe { (*node).right }, data)
    }

    /// Insert a node in the current subtree
    fn insert_in_node(mut node: *mut AVLTreeNode<T>, tmp: *mut AVLTreeNode<T>) {
        if node.is_null() {
            node = tmp;
            return;
        }
        if unsafe { (*tmp).data } < unsafe { (*node).data } { // Insert in left subtree
            Self::insert_in_node(unsafe { (*node).left }, tmp);
            if Self::get_height(unsafe { (*node).left })
                - Self::get_height(unsafe { (*node).right })
                == 2
            {
                if unsafe { (*tmp).data } < unsafe { &*(*node).left }.data {
                    Self::ll_rotate(node);
                } else {
                    Self::lr_rotate(node);
                }
            }
        } else if unsafe { (*tmp).data } > unsafe { (*node).data } { // Insert in right subtree
            Self::insert_in_node(unsafe { (*node).right }, tmp);
            if Self::get_height(unsafe { (*node).right })
                - Self::get_height(unsafe { (*node).left })
                == 2
            {
                if unsafe { (*tmp).data } > unsafe { &*(*node).right }.data {
                    Self::rr_rotate(node);
                } else {
                    Self::rl_rotate(node);
                }
            }
        }
        unsafe {
            (*node).height = max( // Update the height of the current node
                Self::get_height((*node).left),
                Self::get_height((*node).right),
            ) + 1;
        }
    }

    /// Adjust the height after removing a node
    fn adjust(mut node: *mut AVLTreeNode<T>, sub_tree: bool) -> bool {
        let left_height = Self::get_height(unsafe { (*node).left });
        let right_height = Self::get_height(unsafe { (*node).right });
        if sub_tree { // Delete on right sub tree to make it shorter
            if left_height - right_height == 1 {
                return true;
            }
            if left_height == right_height {
                unsafe {
                    (*node).height -= 1;
                    return false;
                }
            }
            if Self::get_height(unsafe { (*(*node).left).right }) > Self::get_height(unsafe { (*(*node).left).left }) {
                Self::lr_rotate(node);
                false
            } else {
                Self::ll_rotate(node);
                Self::get_height(unsafe { (*node).right }) == Self::get_height(unsafe { (*node).left })
            }
        } else { // Delete on left sub tree to make it shorter
            if right_height - left_height == 1 {
                return true;
            }
            if left_height == right_height {
                unsafe {
                    (*node).height -= 1;
                    return false;
                }
            }
            if Self::get_height(unsafe { (*(*node).right).left }) > Self::get_height(unsafe { (*(*node).right).right }) {
                Self::rl_rotate(node);
                false
            } else {
                Self::rr_rotate(node);
                Self::get_height(unsafe { (*node).right }) == Self::get_height(unsafe { (*node).left })
            }

        }
    }
        

    fn delete_in_node(mut node: *mut AVLTreeNode<T>, data: T) -> bool {
        if node.is_null() {
            return true;
        }
        if data == unsafe { (*node).data } { // Delete current node
            if !unsafe { (*node).left }.is_null() && !unsafe { (*node).right }.is_null() {
                let mut tmp = unsafe { (*node).right };
                while !unsafe { (*tmp).left }.is_null() {
                    tmp = unsafe { (*tmp).left };
                }
                unsafe {
                    (*tmp).left = (*node).left;
                    (*tmp).right = (*node).right;
                    (*tmp).height = (*node).height;
                }
                if Self::delete_in_node(unsafe { (*node).right }, unsafe { (*tmp).data }) {
                    true
                } else {
                    Self::adjust(node, false)
                }
            }
            else {
                let tmp = node;
                if unsafe { (*node).left }.is_null() {
                    node = unsafe { (*node).right };
                } else {
                    node = unsafe { (*node).left };
                }
                unsafe {
                    core::ptr::drop_in_place(tmp);
                }
                false
            }
        } else if (data < unsafe { (*node).data } && Self::delete_in_node(unsafe { (*node).left }, data))
            || (data > unsafe { (*node).data } && Self::delete_in_node(unsafe { (*node).right }, data))
        {
            true
        } else {
            Self::adjust(node, data > unsafe { (*node).data })
        }
    }
}
