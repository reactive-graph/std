const path = require('node:path');
const webpack = require('webpack');
const CopyWebpackPlugin = require('copy-webpack-plugin')
const HtmlWebpackPlugin = require('html-webpack-plugin')

module.exports = {
  devtool: 'cheap-source-map',
  performance: {
    hints: false,
  },
  mode: 'development',
  devServer: {
    port: 31416,
    proxy: {
      '/graphql': {
        target: 'http://localhost:31415'
      },
      '/dynamic_graph': {
        target: 'http://localhost:31415'
      }
    },
    liveReload: true,
  },
  resolve: {
    extensions: ['.ts', '.tsx', '.mjs', '.js', '.json', '.css', '.svg'],
  },
  entry: ['./index.jsx'],
  output: {
    path: path.resolve(__dirname, 'dist'),
    filename: '[name].js',
    sourceMapFilename: '[file].map',
  },
  module: {
    rules: [
      {
        test: /\.jsx?$/,
        loader: 'babel-loader',
        exclude: /node_modules/,
        options: {
          presets: ['@babel/env', '@babel/react'],
        },
      },
      {
        test: /\.css$/i,
        use: [
          'style-loader',
          'css-loader',
        ],
      },
    ],
  },
  plugins: [
    new webpack.LoaderOptionsPlugin({
      minimize: true,
      debug: false,
    }),
    new HtmlWebpackPlugin({
      template: path.resolve(__dirname, './index.html'),
      filename: 'index.html',
      chunks: ['index'],
    }),
    new CopyWebpackPlugin({
      patterns: [
        {
          from: './node_modules/graphql-voyager/dist/voyager.css',
          to: 'assets/css'
        },
        {
          from: './assets',
          to: 'assets/'
        },
      ]
    }),
  ],
}
