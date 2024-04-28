// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use core::cmp::{max, Ordering};

#[derive(Clone, Copy)]
pub struct AVLTree(usize);

/// AVL Tree Node
///
/// This struct is used to represent a node in AVL Tree. The `data` field may be replaced by the address of the node.
struct AVLTreeNode {
    data: usize,
    height: usize,
    left: *mut AVLTreeNode,
    right: *mut AVLTreeNode,
}

impl AVLTreeNode {
    fn init(&mut self, data: usize) {
        self.data = data;
        self.height = 1;
        self.left = core::ptr::null_mut();
        self.right = core::ptr::null_mut();
    }
}

impl AVLTree {
    /// Get the height of the current node
    fn get_height(node: *mut AVLTreeNode) -> usize {
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
    fn ll_rotate(node: *mut AVLTreeNode) -> *mut AVLTreeNode {
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
            new_root
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
    fn rr_rotate(node: *mut AVLTreeNode) -> *mut AVLTreeNode {
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
            new_root
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
    fn lr_rotate(node: *mut AVLTreeNode) -> *mut AVLTreeNode {
        unsafe {
            (*node).left = Self::rr_rotate((*node).left);
            Self::ll_rotate(node)
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
    fn rl_rotate(node: *mut AVLTreeNode) -> *mut AVLTreeNode {
        unsafe {
            (*node).right = Self::ll_rotate((*node).right);
            Self::rr_rotate(node)
        }
    }

    /// Find a node in the current subtree
    fn find_in_node(node: *mut AVLTreeNode, data: usize) -> bool {
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
    fn insert_in_node(mut node: *mut AVLTreeNode, tmp: *mut AVLTreeNode) -> *mut AVLTreeNode {
        if node.is_null() {
            return tmp;
        }
        match unsafe { (*tmp).data.cmp(&(*node).data) } {
            Ordering::Less => unsafe {
                // Insert in left subtree
                (*node).left = Self::insert_in_node((*node).left, tmp);
                if Self::get_height((*node).left) - Self::get_height((*node).right) == 2 {
                    if (*tmp).data < (*(*node).left).data {
                        node = Self::ll_rotate(node)
                    } else {
                        node = Self::lr_rotate(node)
                    }
                }
            },
            Ordering::Greater => unsafe {
                // Insert in right subtree
                (*node).right = Self::insert_in_node((*node).right, tmp);
                if Self::get_height((*node).right)
                    - Self::get_height((*node).left)
                    == 2
                {
                    if (*tmp).data > (*(*node).right).data {
                        node = Self::rr_rotate(node);
                    } else {
                        node = Self::rl_rotate(node);
                    }
                }
            }
            Ordering::Equal => {}
        }
        unsafe {
            (*node).height = max(
                // Update the height of the current node
                Self::get_height((*node).left),
                Self::get_height((*node).right),
            ) + 1;
        }
        node
    }

    /// Adjust the height after removing a node
    fn adjust(node: *mut AVLTreeNode, sub_tree: bool) -> (bool, *mut AVLTreeNode) {
        let left_height = Self::get_height(unsafe { (*node).left });
        let right_height = Self::get_height(unsafe { (*node).right });
        if sub_tree {
            // Delete on right sub tree to make it shorter
            if left_height - right_height == 1 {
                return (true, node);
            }
            if left_height == right_height {
                unsafe {
                    (*node).height -= 1;
                    return (false, node);
                }
            }
            if Self::get_height(unsafe { (*(*node).left).right })
                > Self::get_height(unsafe { (*(*node).left).left })
            {
                (false, Self::lr_rotate(node))
            } else {
                (
                    Self::get_height(unsafe { (*node).right })
                        == Self::get_height(unsafe { (*node).left }),
                    Self::ll_rotate(node),
                )
            }
        } else {
            // Delete on left sub tree to make it shorter
            if right_height - left_height == 1 {
                return (true, node);
            }
            if left_height == right_height {
                unsafe {
                    (*node).height -= 1;
                    return (false, node);
                }
            }
            if Self::get_height(unsafe { (*(*node).right).left })
                > Self::get_height(unsafe { (*(*node).right).right })
            {
                (false, Self::rl_rotate(node))
            } else {
                (
                    Self::get_height(unsafe { (*node).right })
                        == Self::get_height(unsafe { (*node).left }),
                    Self::rr_rotate(node),
                )
            }
        }
    }

    fn delete_in_node(mut node: *mut AVLTreeNode, data: usize) -> (bool, *mut AVLTreeNode, bool) {
        if node.is_null() {
            return (true, node, false);
        }
        if data == unsafe { (*node).data } {
            // Delete current node
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
                let (mut flag, son, status) =
                    Self::delete_in_node(unsafe { (*node).right }, unsafe { (*tmp).data });
                unsafe {
                    (*node).right = son;
                }
                if !flag {
                    (flag, node) = Self::adjust(node, false);
                }
                (flag, node, status)
            } else {
                let tmp = node;
                if unsafe { (*node).left }.is_null() {
                    node = unsafe { (*node).right };
                } else {
                    node = unsafe { (*node).left };
                }
                unsafe {
                    core::ptr::drop_in_place(tmp);
                }
                (false, node, true)
            }
        } else {
            let (mut flag, son, status) = if data < unsafe { (*node).data } {
                Self::delete_in_node(unsafe { (*node).left }, data)
            } else {
                Self::delete_in_node(unsafe { (*node).right }, data)
            };
            unsafe {
                if data < (*node).data {
                    (*node).left = son;
                } else {
                    (*node).right = son;
                }
            }
            if !flag {
                (flag, node) = Self::adjust(node, data > unsafe { (*node).data });
            }
            (flag, node, status)
        }
    }

    fn delete_min_in_node(mut node: *mut AVLTreeNode) -> (bool, *mut AVLTreeNode, usize) {
        if node.is_null() {
            return (false, node, usize::default());
        }
        unsafe {
            if (*node).left.is_null() {
                (false, (*node).right, (*node).data)
            } else {
                let (mut flag, left, data) = Self::delete_min_in_node((*node).left);
                (*node).left = left;
                if flag {
                    (flag, node) = Self::adjust(node, false);
                }
                (flag, node, data)
            }
        }
    }
}

impl AVLTree {
    /// Create a new AVL Tree
    pub const fn new() -> Self {
        Self(0usize)
    }

    pub fn insert(&mut self, data: usize) {
        let node = data as *mut AVLTreeNode;
        unsafe {
            (*node).data = data;
            (*node).height = 1;
            (*node).left = core::ptr::null_mut();
            (*node).right = core::ptr::null_mut();
        }
        self.0 = AVLTree::insert_in_node(self.0 as *mut AVLTreeNode, node) as usize;
    }

    pub fn find(&self, data: usize) -> bool {
        AVLTree::find_in_node(self.0 as *mut AVLTreeNode, data)
    }

    pub fn delete(&mut self, data: usize) -> bool {
        let (flag, node, _) = AVLTree::delete_in_node(self.0 as *mut AVLTreeNode, data);
        self.0 = node as usize;
        flag
    }

    pub fn pop_min(&mut self) -> Option<usize> {
        if self.is_empty() {
            return None;
        }
        let (_, node, data) = AVLTree::delete_min_in_node(self.0 as *mut AVLTreeNode);
        self.0 = node as usize;
        Some(data)
    }

    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }
}
