fn find_outlier(values: &[i32]) -> i32 {
    let mut odd: i32 = 0;
    let mut even: i32 = 0;

    let mut encountered_odd = false;
    let mut encountered_even = false;
    let mut result_odd = false;
    let mut result_even = false;

    for i in values.iter() {
        if i & 1 == 1 {
            odd = *i;

            if encountered_odd {
                result_even = true;
            }

            encountered_odd = true;
        } else {
            even = *i;

            if encountered_even {
                result_odd = true;
            }

            encountered_even = true;
        }

        if result_odd && encountered_odd {
            return odd;
        }

        if result_even && encountered_even {
            return even;
        }
    }
    unreachable!()
}
