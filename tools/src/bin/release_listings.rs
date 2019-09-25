use std::fs;
use std::path::Path;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let listings_dir = Path::new("listings");
    let out_dir = Path::new("tmp/listings");

    if out_dir.is_dir() {
        fs::remove_dir_all(out_dir)?;
    }

    fs::create_dir(out_dir)?;

    for chapter in fs::read_dir(listings_dir)? {
        let chapter = chapter?;
        let chapter_path = chapter.path();

        let chapter_name = chapter_path.file_name().expect("Chapter should've had a name");

        // Create corresponding chapter dir in tmp
        let output_chapter_path = out_dir.join(chapter_name);
        fs::create_dir(&output_chapter_path)?;

        for listing in fs::read_dir(chapter_path)? {
            let listing = listing?;
            let listing_path = listing.path();

            let listing_name = listing_path.file_name().expect("Listing should've had a name");

            // Create corresponding listing dir in tmp
            let output_listing_dir = output_chapter_path.join(listing_name);
            fs::create_dir(&output_listing_dir)?;

            for item in fs::read_dir(listing_path)? {
                let item = item?;
                let item_path = item.path();

                let item_name = item_path.file_name().expect("Item should've had a name");
                let output_item = output_listing_dir.join(item_name);

                if item_path.is_dir() {
                    // if it's src, cool
                    // if not, wtf
                } else {
                    // Copy any top-level files without modifications
                    fs::copy(item_path, output_item)?;
                }
            }
        }
    }
    //         create src dir
    //         for each file in src dir
    //             if .rs file (all rs files?)
    //                 for each line in file
    //                     if not anchor comment and not snip comment
    //                         if not lib.rs or not empty fn main
    //                             copy line to corresponding file in tmp
    //
    // tar tmp dir
    // println!("Release tarball of listings in {}", tarball);

    Ok(())
}
