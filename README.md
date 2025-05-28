# Cat Lab 🐱

A Rust genetics simulation program that models cat coat color inheritance patterns based on real feline genetics.

## Overview

Cat Lab simulates the complex genetics behind cat coat colors, including:

- **Black/Brown genes** (B locus): Controls black, chocolate, and cinnamon coloring
- **White genes** (W locus): Controls white masking of other colors
- **Orange genes** (O locus): X-linked orange coloration (sex-linked inheritance)

The simulation demonstrates how these genes interact to produce different coat colors and patterns, following real genetic inheritance rules.

## Features

- 🧬 **Realistic Genetics**: Based on actual feline coat color genetics
- 🎲 **Random Cat Generation**: Create cats with randomized genetic profiles
- 👶 **Breeding Simulation**: Breed cats to produce offspring with inherited traits
- 🔬 **Detailed Genetic Information**: View complete genotype information for each cat
- ⚤ **Sex-Linked Inheritance**: Properly models X-linked orange gene inheritance

## Project Structure

    src/
    ├── main.rs          # Main program with breeding demonstrations
    ├── cat.rs           # Cat struct and breeding logic
    └── cat/
        ├── coat_genes.rs    # Genetic allele definitions and coat gene logic
        └── gender.rs        # Gender enum and related functionality

## Genetic Model

### Black/Brown Genes (Autosomal)

- **B (Dominant)**: Black coloration
- **b (Recessive)**: Brown/chocolate coloration  
- **b' (Recessive)**: Cinnamon coloration

### White Genes (Autosomal)

- **W (Dominant)**: White masking (dominant/epistatic over other colors)
- **w (Recessive)**: Allows other colors to show

### Orange Genes (X-Linked)

- **O (Dominant)**: Orange coloration
- **o (Recessive)**: Non-orange coloration

**Note**: Males have only one X chromosome (XY), so they need only one O allele to be orange. Females have two X chromosomes (XX), creating the possibility for tortoiseshell patterns when heterozygous (Oo).

## Usage

### Running the Program

    cargo run

The program will:

1. Generate 5 random cats with detailed genetic information
2. Create two parent cats (Tom and Queen)
3. Demonstrate breeding to produce a kitten
4. Display genetic information for all cats

### Example Output

    Name: Cat 1, Age: 8, Gender: Female, Coat colour: 
    Black genes: B (Dominant Black) and b (Recessive Black)
    White genes: w (Recessive White) and W (Dominant White)
    Orange genes: O (Orange) and o (Non-Orange)

    Parents:
    Father: Name: Tom, Age: 3, Gender: Male, Coat colour: ...
    Mother: Name: Queen, Age: 2, Gender: Female, Coat colour: ...

    Kitten born!
    Kitten: Name: Kitten, Age: 0, Gender: Female, Coat colour: ...

## Breeding Rules

1. **Gender Requirement**: Breeding requires one male and one female cat
2. **Age**: Kittens are born at age 0
3. **Inheritance**: Currently uses simple inheritance (50/50 chance from each parent)
4. **Gender**: Kitten gender is randomly determined

## Dependencies

- `rand`: For random number generation and genetic variation

Add to your `Cargo.toml`:

    [dependencies]
    rand = "0.8"

## License

This project is educational and open source. Feel free to use, modify, and extend for learning purposes.
