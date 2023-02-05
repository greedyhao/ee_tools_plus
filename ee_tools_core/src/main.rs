// use bin_converter::ImageConverter;
use clap::Parser;
use header_syncer::*;

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

        /// Class name; e.g., `--class-name "test"`,
        /// then it will add `// test` to the start of the sync code
        #[arg(long, default_value_t = String::new())]
        class_name: String,

        /// Ignore symbol; e.g., `--ignore-symbol "sym1 sym2"`
        #[arg(long, default_value_t = String::new())]
        ignore_symbol: String,

        /// Symbol compression mode; e.g., `--comp
        #[arg(short, long, default_value_t = true)]
        compress: bool,
    },

    /// Binary format convertor
    BinConverter {
        /// This option is used to specify an initialization script file.
        #[arg(long, default_value_t = String::new())]
        init: String,

        /// Format before conversion; e.g., `--from "text.txt"`
        #[arg(long)]
        from: String,

        /// Format after conversion; e.g., `--to "image.png"
        #[arg(long)]
        to: String,

        #[arg(long, default_value_t = String::new())]
        width: String,

        #[arg(long, default_value_t = String::new())]
        height: String,

        #[arg(long, default_value_t = String::new())]
        rgb_type: String,

        #[arg(long, default_value_t = false)]
        has_alpha: bool,

        #[arg(long, default_value_t = String::new())]
        out_format: String,
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
        } => {
            let from = from.split(' ').collect();
            let to = to.split(' ').collect();
            let isyms = ignore_symbol.split(' ').collect();
            let mut syncer = Syncer::new(from, to, &sync_lable);

            if type_of_from == "gnu_lds" {
                syncer.set_type_of_form(FromFileType::GnuLinkScript);
            }

            syncer.set_class_name(&class_name);
            syncer.set_ignore_symbols(isyms);
            // syncer.set_mark_symbols(mark)
            syncer.set_compress(compress);
            syncer.run();
        }
        Action::BinConverter {
            init: _,
            from,
            to,
            width,
            height,
            rgb_type: _,
            has_alpha: _,
            out_format: _,
        } => {
            let w: u32;
            let h: u32;
            let mut from_s = from.split('.').collect::<Vec<&str>>();
            let mut to_s = to.split('.').collect::<Vec<&str>>();

            // Prevents from[1] and to[1] being empty
            from_s.push("");
            to_s.push("");

            println!("from: {:?} {:?}", from_s, to_s);
            if from_s[1] == "txt" {
                let width = width.parse::<u32>();
                w = match width {
                    Ok(w) => w,
                    Err(_) => panic!("Need to specify the width of the image!"),
                };

                let height = height.parse::<u32>();
                h = match height {
                    Ok(h) => h,
                    Err(_) => panic!("Need to specify the height of the image!"),
                };

                bin_converter::gen_img_from_file(&from, &to, w, h);
                // let img_cov = ImageConverter {
                //     from,
                //     to,
                //     width: w as usize,
                //     height: h as usize,
                // };

                println!("width: {:?} height: {:?}", w, h);
            }

            // let file = args.next().unwrap().split('=').next_back().unwrap();

            // let out = file.split('.').next().unwrap();
            // let out = out.to_owned() + ".png";
            // println!("w={} h={} file={} out={}", width, height, file, out);
        }
    }
}
