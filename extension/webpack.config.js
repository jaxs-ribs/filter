const path = require('path');

module.exports = {
  mode: 'development', // Use 'production' for production builds
  entry: './contentScript.js', 
  devtool: 'cheap-module-source-map',
  output: {
    path: path.resolve(__dirname, 'dist'), // Output directory
    filename: 'bundle.js' // Output file
  },
  module: {
    rules: [
      {
        test: /\.js$/, // Transpile .js files
        exclude: /node_modules/,
        use: {
          loader: 'babel-loader',
          options: {
            presets: ['@babel/preset-env']
          }
        }
      },
      {
        test: /\.(png|jpg|gif)$/i, // Handle image assets
        use: [
          {
            loader: 'file-loader',
          },
        ],
      },
    ]
  }
};