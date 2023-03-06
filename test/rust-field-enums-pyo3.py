from rust_field_enums_pyo3 import schedule, calculate_day
from rust_field_enums_pyo3 import Weekday
from rust_field_enums_pyo3 import Str, Int, process_prop, print_str, print_int
from datetime import date

schedule(Weekday.Monday)
schedule(Weekday.Wednesday)
schedule(Weekday.Saturday)
schedule(Weekday.Sunday)

assert calculate_day(date(2020, 5, 19)) == Weekday.Tuesday

s = Str("pomtery")
print(process_prop(s))
print_str(s)

i = Int(12345)
print(process_prop(i))
print_int(i)
