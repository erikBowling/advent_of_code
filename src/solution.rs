use anyhow::Result;

pub trait Solution {
    fn part1(&self, input: &str) -> Result<String>;
    fn part2(&self, input: &str) -> Result<String>;

    fn run(&self, input: &str) -> Result<()> {
        println!("\n=== Part 1 ===");
        let part1_result = self.part1(input)?;
        println!("Result: {}", part1_result);

        println!("\n=== Part 2 ===");
        let part2_result = self.part2(input)?;
        println!("Result: {}", part2_result);

        Ok(())
    }
}
