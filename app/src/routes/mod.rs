pub mod home;
pub mod blog_list;
pub mod blog_post;
pub mod page_not_found;

#[derive(Clone, Copy, PartialEq)]
pub enum Routes {
   Home,
   Blog,
   Projects,
}
