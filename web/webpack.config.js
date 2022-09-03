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
      },
    }
  },
  resolve: {
    extensions: ['.ts', '.tsx', '.mjs', '.js', '.json', '.css', '.svg'],
    alias: {
      // fix 'duplicated react' issue when using npm link
      react: require.resolve('react'),
      path: require.resolve('path-browserify'),
    },
    fallback: {
      fs: false,
      path: require.resolve('path-browserify')
    },
  },
  entry: {
    'graph': [path.resolve(__dirname, 'graph.tsx')],
    'dynamic-graph': [path.resolve(__dirname, 'dynamic-graph.tsx')],
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
        test: /\.tsx?$/,
        use: 'ts-loader',
        exclude: /node_modules/,
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
          from: './node_modules/altair-static/build/dist/*.js',
          to: '[name][ext]'
        },
        {
          from: './node_modules/altair-static/build/dist/assets',
          to: 'assets/'
        },
        {
          from: './assets',
          to: 'assets/'
        },
      ]
    }),
    new HtmlWebpackPlugin({
      template: path.resolve(__dirname, 'graph.html'),
      filename: 'graph.html',
      chunks: ['graph'],
    }),
    new HtmlWebpackPlugin({
      template: path.resolve(__dirname, 'dynamic-graph.html'),
      filename: 'dynamic-graph.html',
      chunks: ['dynamic-graph'],
    }),
  ],
}
