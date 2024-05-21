use validator::ValidationErrors;

pub fn get_validation_errors(validation_errors: &ValidationErrors) -> Vec<String> {
    let mut key_errors_vec: Vec<String> = Vec::new();
    let validation_errors = ValidationErrors::field_errors(validation_errors);

    for (key, errors) in validation_errors {
        for this_error in errors {
            let error_code = this_error.code.to_string();

            key_errors_vec.push(format!(
                " '{}' no cumple con el formato de {}",
                key, error_code
            ));
        }
    }

    key_errors_vec
}
