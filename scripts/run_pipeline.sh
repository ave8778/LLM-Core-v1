#!/bin/bash
# Example script to run the LLM‑Core pipeline
set -e

# Step 1: research
agent_run --task "collect_academic" --output artefacts.csv || true
agent_run --task "scrape_github"    --append artefacts.csv || true
agent_run --task "patent_map"       --output patent_map.graphml || true

# Step 2: grammar and lexicon
agent_run --task "generate_bnf"     --input artefacts.csv --output bnf.g4 || true
agent_run --task "hygenar_optim"    --input bnf.g4        --output lexicon_core.tsv || true

# Step 3: build
cargo build -p llmcore_parser || true
python scripts/gad_wrapper.py --model gpt-4o --grammar bnf.g4 || true

# Step 4: test
pytest || true
agent_run --task "fuzz_test" || true

# Step 5: release
agent_run --task "publish_docs" --site docs/ || true
agent_run --task "announce_discord" || true
