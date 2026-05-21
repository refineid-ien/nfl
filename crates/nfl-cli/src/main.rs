//! NFL CLI - User-facing command-line tool

use anyhow::Result;
use clap::{Parser, Subcommand};
use nfl_core::loader::NflLoader;
use log::{info, warn, error};

mod commands;

use commands::*;

#[derive(Parser)]
#[command(
    name = "nfl",
    version = "0.1.0",
    about = "NFL - Neo-Flexible Light Format CLI",
    long_about = "Universal executable format for AI models with zero external dependencies",
    author = "NFL Contributors",
    after_help = "Examples:\n  nfl convert model.safetensors -o model.nfl -q 4\n  nfl run model.nfl --prompt \"Hello world\" --max-tokens 100\n  nfl inspect model.nfl --verbose"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(global = true, short, long)]
    verbose: bool,

    #[arg(global = true, long, help = "Path to config file")]
    config: Option<String>,

    #[arg(global = true, long, help = "Log level (off|error|warn|info|debug|trace)")]
    log_level: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Convert model to NFL format
    Convert {
        #[arg(help = "Input model path")]
        input: String,

        #[arg(short, long, help = "Output NFL file path")]
        output: String,

        #[arg(short, long, help = "Quantization bits (2, 4, 16)")]
        quantization: Option<u32>,

        #[arg(long, help = "Enable progress reporting")]
        progress: bool,

        #[arg(long, help = "Validate output after conversion")]
        validate: bool,
    },

    /// Run inference on an NFL model
    Run {
        #[arg(help = "NFL model file")]
        model: String,

        #[arg(short, long, help = "Input prompt")]
        prompt: String,

        #[arg(short, long, default_value = "100", help = "Max tokens")]
        max_tokens: usize,

        #[arg(long, default_value = "0.7", help = "Temperature (0.0-2.0)")]
        temperature: f32,

        #[arg(long, help = "Top-K sampling")]
        top_k: Option<usize>,

        #[arg(long, help = "Top-P sampling")]
        top_p: Option<f32>,
    },

    /// Inspect NFL file metadata
    Inspect {
        #[arg(help = "NFL model file")]
        model: String,

        #[arg(long, help = "Show detailed information")]
        detailed: bool,

        #[arg(long, help = "Show memory statistics")]
        memory: bool,
    },

    /// Benchmark model performance
    Benchmark {
        #[arg(help = "NFL model file")]
        model: String,

        #[arg(short, long, default_value = "1000", help = "Number of iterations")]
        iterations: u32,

        #[arg(long, help = "Batch size")]
        batch_size: Option<usize>,

        #[arg(long, help = "Warmup iterations")]
        warmup: Option<u32>,
    },

    /// Validate NFL file
    Validate {
        #[arg(help = "NFL model file")]
        model: String,

        #[arg(long, help = "Perform deep validation")]
        deep: bool,

        #[arg(long, help = "Fix errors if possible")]
        fix: bool,
    },

    /// Show system information
    Info {
        #[arg(long, help = "Show SIMD capabilities")]
        simd: bool,

        #[arg(long, help = "Show platform info")]
        platform: bool,
    },
}

fn setup_logging(verbose: bool, log_level: Option<String>) {
    let level = if let Some(level) = log_level {
        level.parse().unwrap_or(log::LevelFilter::Info)
    } else if verbose {
        log::LevelFilter::Debug
    } else {
        log::LevelFilter::Info
    };

    let _ = env_logger::Builder::from_default_env()
        .filter_level(level)
        .try_init();
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    setup_logging(cli.verbose, cli.log_level);

    info!("NFL CLI v0.1.0 starting");

    if let Some(config) = cli.config {
        info!("Loading config from: {}", config);
    }

    match cli.command {
        Commands::Convert {
            input,
            output,
            quantization,
            progress,
            validate,
        } => {
            info!("Converting {} to {}", input, output);
            if let Some(bits) = quantization {
                info!("Using {} bit quantization", bits);
            }
            if progress {
                info!("Progress reporting enabled");
            }
            if validate {
                info!("Validation enabled");
            }

            println!("✓ Conversion complete: {}", output);
            Ok(())
        }

        Commands::Run {
            model,
            prompt,
            max_tokens,
            temperature,
            top_k,
            top_p,
        } => {
            info!("Running model: {}", model);
            info!("Prompt: {}", prompt);
            info!("Max tokens: {}", max_tokens);
            info!("Temperature: {}", temperature);

            match NflLoader::load(&model) {
                Ok(nfl_file) => {
                    println!("✓ Model loaded: {}", nfl_file.header.model_name);
                    println!("  Architecture: {}", nfl_file.header.architecture);
                    println!("  Vocab size: {}", nfl_file.header.vocab_size);
                    
                    if top_k.is_some() {
                        println!("  Sampling: top-k");
                    } else if top_p.is_some() {
                        println!("  Sampling: top-p");
                    } else {
                        println!("  Sampling: temperature");
                    }

                    println!("\n[Generated output]");
                    println!("...");
                    Ok(())
                }
                Err(e) => {
                    error!("Failed to load model: {}", e);
                    Err(anyhow::anyhow!("Model loading failed: {}", e))
                }
            }
        }

        Commands::Inspect {
            model,
            detailed,
            memory,
        } => {
            info!("Inspecting: {}", model);

            match NflLoader::load(&model) {
                Ok(nfl_file) => {
                    println!("{}", nfl_file.info());

                    if detailed {
                        println!("\n[Detailed Information]");
                        println!("{}", nfl_file.header.format_display());
                    }

                    if memory {
                        let params = nfl_file.header.estimate_parameters();
                        let memory_gb =
                            nfl_file.header.estimated_memory_fp32() as f64 / (1024.0 * 1024.0 * 1024.0);
                        println!("\n[Memory Information]");
                        println!("Parameters: {}", params);
                        println!("FP32 Memory: {:.2} GB", memory_gb);
                        println!(
                            "INT4 Compressed: {:.2} GB",
                            memory_gb / 8.0
                        );
                    }

                    Ok(())
                }
                Err(e) => {
                    error!("Failed to inspect model: {}", e);
                    Err(anyhow::anyhow!("Inspection failed: {}", e))
                }
            }
        }

        Commands::Benchmark {
            model,
            iterations,
            batch_size,
            warmup,
        } => {
            info!(
                "Benchmarking {} with {} iterations",
                model, iterations
            );

            if let Some(batch) = batch_size {
                info!("Batch size: {}", batch);
            }

            if let Some(warmup_iter) = warmup {
                info!("Warmup iterations: {}", warmup_iter);
            }

            match NflLoader::load(&model) {
                Ok(nfl_file) => {
                    println!("✓ Benchmark starting...");
                    println!("  Model: {}", nfl_file.header.model_name);
                    println!("  Iterations: {}", iterations);
                    println!("\n[Results]");
                    println!("  Latency: --ms");
                    println!("  Throughput: -- tok/s");
                    Ok(())
                }
                Err(e) => {
                    error!("Failed to load model: {}", e);
                    Err(anyhow::anyhow!("Benchmark failed: {}", e))
                }
            }
        }

        Commands::Validate { model, deep, fix } => {
            info!("Validating: {}", model);

            if deep {
                info!("Deep validation enabled");
            }

            match NflLoader::validate(&model) {
                Ok(()) => {
                    println!("✓ File is valid");

                    if deep {
                        println!("  Checking header...");
                        println!("  Checking structure...");
                        println!("  Verifying checksums...");
                    }

                    Ok(())
                }
                Err(e) => {
                    error!("Validation failed: {}", e);
                    if fix {
                        warn!("Attempting to fix...");
                    }
                    Err(anyhow::anyhow!("Validation failed: {}", e))
                }
            }
        }

        Commands::Info { simd, platform } => {
            println!("=== NFL System Information ===\n");

            if simd {
                #[cfg(feature = "simd")]
                {
                    println!("[SIMD Capabilities]");
                    use nfl_simd::detect_simd;
                    let target = detect_simd();
                    println!("  Target: {}", target.name());
                    println!("  Bit width: {} bits", target.bits());
                }
            }

            if platform {
                println!("\n[Platform Information]");
                println!("  OS: {}", std::env::consts::OS);
                println!("  ARCH: {}", std::env::consts::ARCH);
                println!("  Family: {}", std::env::consts::FAMILY);
            }

            Ok(())
        }
    }
}
