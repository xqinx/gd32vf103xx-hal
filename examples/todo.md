## Some improvements that might help

### modules had been reviewed

* afio
* backup_domain
* delay
* eclic
* exmc
* gpio
* lib
* prelude
* rcu
* signature
* timer
* time
* watchdog

pwm
i2c
rtc
serial
spi

TODO:
* afio: add support for EXTI pin selection
* delay: add support for <1Hz timer-based delay
         improve ergonomnics of creating timer based delays


### Timer based delay
* Corrently only accept delays equal or smaller than 1 second, since it's using
the timer Countdown and timer's start() function accept <IntoHertz> as an
argument, therefore anything below 1.hz() will cause a divide by zero error

* When creating timer based delay it will `start()` the timer, would this cause
any issue? e.g. the timer is started by not used until later, and the intial
timeout occurs before using it later, cause UIF incorrectly set
  Yes, and we should fix it by clearing `upif` before enabling cnt
  DONE

* we need to use UE for timers reload since simply `reset` counter would not
  clear the counter. Moreover if ARSE is enable, car nor psc would not be
updated until next UEV
  DONE

* improve ergonomnics of creating delay using timers
