# Weather Forecaster App

This is a simple command-line weather app written in Rust. It retrieves weather data from the OpenWeatherMap API based on latitude and longitude coordinates.

## Installation

To install the app, you will need to have Rust and Cargo installed on your system. Then, clone the repository and run the following command:

```
cargo build --release
```

This will build the app in release mode and create an executable in the `target/release` directory.

## Usage

To use the app, run the following command:

```
./target/release/weather-forecaster LATITUDE LONGITUDE
```

Replace `LATITUDE` and `LONGITUDE` with the desired values.


### Examples

Here are some examples of how to use the app:

```
./target/release/weather-forecaster 37.7749 -122.4194
```

This will retrieve weather data for San Francisco, CA.

## Configuration

The app can be configured using `api_key`.

- `WEATHER_API_KEY`: The API key for the OpenWeatherMap API. This can be obtained by creating an account on the [OpenWeatherMap website](https://openweathermap.org/).

## Contributing

Contributions are welcome! If you find a bug or have a feature request, please open an issue on GitHub. If you would like to contribute code, please fork the repository and submit a pull request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
