import field_enums
from field_enums import Weekday
from datetime import date

field_enums.schedule(Weekday.Monday)
field_enums.schedule(Weekday.Wednesday)
field_enums.schedule(Weekday.Friday)
field_enums.schedule(Weekday.Saturday)
field_enums.schedule(Weekday.Sunday)

assert field_enums.calculate_day(date(2020, 5, 19)) == Weekday.Tuesday
