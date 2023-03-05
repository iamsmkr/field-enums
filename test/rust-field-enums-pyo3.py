from rust_field_enums_pyo3 import schedule, calculate_day
from rust_field_enums_pyo3 import Weekday
from datetime import date

schedule(Weekday.Monday)
schedule(Weekday.Wednesday)
schedule(Weekday.Saturday)
schedule(Weekday.Sunday)

assert calculate_day(date(2020, 5, 19)) == Weekday.Tuesday
