# citybike-stations-cli

A Rust excercise. A command line app for fetching the status of the nearest city bike (kaupunkipyörä) stations by address, place or landmark. Uses the [Digitransit api](https://digitransit.fi/en/developers/) for geocoding & fetching station information.

## Usage

```
./citybike-stations "rautatientori"

Etäisyys        Nimi                            Pyöriä/paikkoja
------------------------------------------------------------------
73m             Rautatientori / itä             4/20
220m            Rautatientori / länsi           1/17
240m            Kaisaniemenpuisto               4/16
300m            Töölönlahdenkatu                8/46
```
