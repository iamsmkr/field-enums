from rust_field_enums_pyo3 import schedule, calculate_day
from rust_field_enums_pyo3 import Weekday
from rust_field_enums_pyo3 import Str, Int, print_prop
from datetime import date

schedule(Weekday.Monday)
schedule(Weekday.Wednesday)
schedule(Weekday.Saturday)
schedule(Weekday.Sunday)

assert calculate_day(date(2020, 5, 19)) == Weekday.Tuesday

s = Str("pomtery")
print_prop(s)

i = Int(12345)
print_prop(i)
