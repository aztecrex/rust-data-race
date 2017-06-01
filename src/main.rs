mod new_owner;
mod copyer;
mod borrower;
mod concurrent;

fn main() {
    new_owner::demo();
    copyer::demo();
    borrower::demo();
    concurrent::demo_naiive();
    concurrent::demo_will_not_work();
    concurrent::demo_still_will_not_work();
    concurrent::demo();
}
