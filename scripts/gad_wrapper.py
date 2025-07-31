"""
Grammar‑Aligned Decoder (GAD) wrapper for OpenAI and LLaMA‑cpp models.
This script demonstrates how to enforce a grammar while generating text.
"""

import argparse

# Placeholder for actual implementation

def main():
    parser = argparse.ArgumentParser(description="Run GAD with given grammar")
    parser.add_argument("--model", required=True, help="model name e.g. gpt-4o or llama-cpp")
    parser.add_argument("--grammar", required=True, help="path to BNF grammar file")
    args = parser.parse_args()
    print(f"Running GAD wrapper with model {args.model} and grammar {args.grammar}")
    # Implementation would load model, compile grammar, and generate text

if __name__ == "__main__":
    main()
