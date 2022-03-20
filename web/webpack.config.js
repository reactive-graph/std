const webpack = require('webpack');
const CopyWebpackPlugin = require('copy-webpack-plugin')
const HtmlWebpackPlugin = require('html-webpack-plugin')
const path = require('path')

module.exports = {
  mode: 'development',
  devServer: {
    port: 31416,
    proxy: {
      "/graphql": {
        target: "http://localhost:31415"
      }
    }
  },
  resolve: {
    extensions: ['.ts', '.tsx', '.mjs', '.js', '.json', '.css', '.svg'],
    alias: {
      // fix "duplicated react" issue when using npm link
      react: require.resolve('react'),
    },
  },
  entry: {
    query: [path.resolve(__dirname, "query.jsx")],
    mutation: [path.resolve(__dirname, "mutation.jsx")],
    subscription: [path.resolve(__dirname, "subscription.jsx")],
  },
  output: {
    filename: '[name].js',
    path: path.resolve(__dirname, 'dist', 'bundle'),
  },

  module: {
    rules: [
      {
        test: /.jsx?$/,
        loader: 'babel-loader',
        exclude: /node_modules/,
        options: {
          presets: ['@babel/env', '@babel/react'],
        },
      },
      {
        test: /\.css$/i,
        use: [
          "style-loader",
          "css-loader",
        ],
      },
    ],
  },
  plugins: [
    new webpack.LoaderOptionsPlugin({
      minimize: true,
      debug: false,
    }),
    new CopyWebpackPlugin({
      patterns: [
        {
          from: './node_modules/graphql-voyager/dist/voyager.worker.js',
        },
        {
          from: './assets',
          to: 'assets/'
        },
      ]
    }),
    new HtmlWebpackPlugin({
      template: path.resolve(__dirname, 'query.html'),
      filename: 'query.html',
      chunks: ['query'],
    }),
    new HtmlWebpackPlugin({
      template: path.resolve(__dirname, 'mutation.html'),
      filename: 'mutation.html',
      chunks: ['mutation'],
    }),
    new HtmlWebpackPlugin({
      template: path.resolve(__dirname, 'subscription.html'),
      filename: 'subscription.html',
      chunks: ['subscription'],
    }),
  ],
}
