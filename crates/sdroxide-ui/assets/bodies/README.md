# Body maps

`moon.jpg` — 2048×1024 equirectangular albedo map of the Moon, −180°…180° east
longitude with 0° at the centre and +90°…−90° latitude, matching the sphere
mesh's UVs and `sdroxide_solar::ephem::moon_basis`'s frame directly.

Source: NASA/Goddard Scientific Visualization Studio, *CGI Moon Kit*
(<https://svs.gsfc.nasa.gov/4720>), `lroc_color_poles_2k.tif` — the LRO Wide
Angle Camera global albedo mosaic. NASA imagery is public domain.

Re-encoded to JPEG at quality 88 (3.2 MB TIFF → 0.5 MB) because it is a
photograph: PNG of the same data is four times the size for no visible gain,
and this is the one body in the view whose real surface everybody already
knows by sight.

`mars.jpg` — 2048×1024, same layout, built by `make_body_maps.py`.

Source: MDIM 2.1, the USGS Viking Mars Digital Image Mosaic tied to the MOLA
control network at 256 pixel/degree and colourised by NASA Ames, requested from
the USGS planetary WMS (`planetarymaps.usgs.gov`, layer `MDIM21_color`) at
4096×2048 and filtered down. USGS and NASA imagery is public domain.

The request settles the projection: `SRS=EPSG:4326` with
`BBOX=-180,-90,180,90` is the texture's own grid, so nothing is re-projected
afterwards and Syrtis Major, Valles Marineris and Olympus Mons land at the
longitudes `planets.rs`'s IAU rotation elements put them.

`jupiter.jpg`, `saturn.jpg` — 2048×1024, same layout, built by
`make_body_maps.py` from published Cassini map data:

* Jupiter — PIA07782, *Cassini's Best Maps of Jupiter* (December 2000),
  <https://images-assets.nasa.gov/image/PIA07782/PIA07782~orig.jpg>.
* Saturn — the Cassini ISS RGB global colour map of 11 August 2011, from the
  PDS Atmospheres node's `co_iss_global-maps` bundle. Saturn's rings hide a
  band of the planet from Cassini's viewpoint and neither pole was in view, so
  those latitudes are interpolated from the rows either side — sound because
  Saturn is zonal, and noted here because it is a reconstruction.

Both are NASA/JPL-Caltech/Space Science Institute, public domain. The script
documents the tone mapping it applies; the published data is calibrated
reflectance and looks washed out shown raw.
