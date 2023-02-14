use bin_converter::*;
use clap::Parser;
use header_syncer::*;
use std::env;

/// EE_TOOLS
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    action: Action,
}

#[derive(clap::Subcommand)]
enum Action {
    /// A synchronous header file program
    HeaderSyncer {
        // /// Set the source file and output file
        // #[arg(short, default_value_t = String::new())]
        // set: String,

        // /// This option is used to specify an initialization script file.
        // #[arg(long, default_value_t = String::new())]
        // init: String,

        // /// Quiet
        // #[arg(short, default_value_t = false)]
        // quiet: bool,
        /// From files; e.g., `--from "api1.h api2.h"`
        #[arg(long)]
        from: String,

        /// To files; e.g., `--to "api.h test.h"`
        #[arg(long)]
        to: String,

        /// Type of From files; e.g., `--type_of_from "gnu_lds"
        #[arg(long, default_value_t = String::new())]
        type_of_from: String,

        /// Sync label; e.g., `--sync-lable "/* header-sync */"`,
        /// then it will copy from '/* header-sync start */' to '/* header-sync end */'
        #[arg(long, default_value_t = String::from("/* header-sync */"))]
        sync_lable: String,

        /// TODO: Class name; e.g., `--class-name "test"`,
        /// then it will add `// test` to the start of the sync code
        #[arg(long, default_value_t = String::new())]
        class_name: String,

        /// TODO: Ignore symbol; e.g., `--ignore-symbol "sym1 sym2"`
        #[arg(long, default_value_t = String::new())]
        ignore_symbol: String,

        /// TODO: Symbol compression mode; e.g., `--comp
        #[arg(short, long, default_value_t = true)]
        compress: bool,

        /// Add additional path variables; e.g., `--extra_path_var path_to\gcc`
        #[arg(long, default_value_t = String::new())]
        extra_path_var: String,
    },

    /// File format convertor
    Converter {
        /// This option is used to specify an initialization script file.
        #[arg(long, default_value_t = String::new())]
        init: String,

        /// Format before conversion; e.g., `--from "text.txt"`
        #[arg(long)]
        from: String,

        /// Format after conversion; e.g., `--to "image.png"
        #[arg(long)]
        to: String,

        /// Width is needed for conversion from 'txt' and binary format
        #[arg(long, default_value_t = String::new())]
        width: String,

        /// Height is needed for conversion from 'txt' and binary format
        #[arg(long, default_value_t = String::new())]
        height: String,

        #[arg(long, default_value_t = String::new())]
        rgb_type: String,

        #[arg(long, default_value_t = false)]
        has_alpha: bool,

        #[arg(long, default_value_t = false)]
        has_custom_format: bool,

        #[arg(long, default_value_t = 0)]
        bits_per_sample: u8,
    },
}

fn main() {
    let args = Args::parse();

    match args.action {
        Action::HeaderSyncer {
            // set,
            // init,
            // quiet,
            from,
            to,
            type_of_from,
            sync_lable,
            class_name,
            ignore_symbol,
            compress,
            extra_path_var,
        } => {
            let from = from.split(' ').collect();
            let to = to.split(' ').collect();
            let isyms = ignore_symbol.split(' ').collect();
            let mut syncer = Syncer::new(from, to, &sync_lable);

            // Add additional path variables
            let mut sys_path = env::var_os("path").unwrap();
            sys_path.push(";");
            sys_path.push(extra_path_var);
            env::set_var("path", sys_path);

            if type_of_from == "gnu_lds" {
                syncer.set_type_of_form(FromFileType::GnuLinkScript);
            }

            syncer.set_class_name(&class_name);
            syncer.set_ignore_symbols(isyms);
            // syncer.set_mark_symbols(mark)
            syncer.set_compress(compress);
            syncer.run();
        }
        Action::Converter {
            init: _,
            from,
            to,
            width,
            height,
            rgb_type: _,
            has_alpha: _,
            has_custom_format,
            bits_per_sample,
        } => {
            let mut converter = ImageConverter::new(from, to, has_custom_format);
            let format = BinFileFormat { bits_per_sample };

            converter.set_width_and_height(width, height);
            converter.set_bin_file_format(format);
            converter.run().unwrap();
        }
    }
}
