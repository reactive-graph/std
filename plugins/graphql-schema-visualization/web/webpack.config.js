const webpack = require('webpack');
const CopyWebpackPlugin = require('copy-webpack-plugin')
const HtmlWebpackPlugin = require('html-webpack-plugin')
const path = require('path')

module.exports = {
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
    'graph/query': [path.resolve(__dirname, 'graph', 'query.jsx')],
    'graph/mutation': [path.resolve(__dirname, 'graph', 'mutation.jsx')],
    'graph/subscription': [path.resolve(__dirname, 'graph', 'subscription.jsx')],
    'dynamic-graph/query': [path.resolve(__dirname, 'dynamic-graph', 'query.jsx')],
    'dynamic-graph/mutation': [path.resolve(__dirname, 'dynamic-graph', 'mutation.jsx')],
    'dynamic-graph/subscription': [path.resolve(__dirname, 'dynamic-graph', 'subscription.jsx')],
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
    new CopyWebpackPlugin({
      patterns: [
        {
          from: './node_modules/@guigiani/graphql-voyager/dist/voyager.worker.js',
        },
        {
          from: './assets',
          to: 'assets/'
        },
      ]
    }),
    new HtmlWebpackPlugin({
      template: path.resolve(__dirname, 'graph', 'query.html'),
      filename: 'graph/query.html',
      chunks: ['graph/query'],
    }),
    new HtmlWebpackPlugin({
      template: path.resolve(__dirname, 'graph', 'mutation.html'),
      filename: 'graph/mutation.html',
      chunks: ['graph/mutation'],
    }),
    new HtmlWebpackPlugin({
      template: path.resolve(__dirname, 'graph', 'subscription.html'),
      filename: 'graph/subscription.html',
      chunks: ['graph/subscription'],
    }),
    new HtmlWebpackPlugin({
      template: path.resolve(__dirname, 'dynamic-graph', 'query.html'),
      filename: 'dynamic-graph/query.html',
      chunks: ['dynamic-graph/query'],
    }),
    new HtmlWebpackPlugin({
      template: path.resolve(__dirname, 'dynamic-graph', 'mutation.html'),
      filename: 'dynamic-graph/mutation.html',
      chunks: ['dynamic-graph/mutation'],
    }),
    new HtmlWebpackPlugin({
      template: path.resolve(__dirname, 'dynamic-graph', 'subscription.html'),
      filename: 'dynamic-graph/subscription.html',
      chunks: ['dynamic-graph/subscription'],
    }),
  ],
}
