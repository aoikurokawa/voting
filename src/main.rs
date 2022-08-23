use design_pattern::default::MyConfiguration;
use design_pattern::use_borrowed_types_for_arguments::main_use_borrowed_types_for_arguments;

fn main() {
    main_use_borrowed_types_for_arguments();

    // construct a new instance with default values
    let mut conf = MyConfiguration::default();
    // do smt with conf here
    conf.check = true;
    println!("conf = {:#?}", conf);

    // partial initialization with default values, creates the same instance
    let conf1 = MyConfiguration {
        check: true,
        ..Default::default()
    };
    assert_eq!(conf, conf1);
}
