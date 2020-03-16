const { merge } = require('webpack-merge');
const common = require('./webpack.common.js');
const path = require("path");

const dist = path.resolve(__dirname, 'dist');

module.exports = merge(common, {
    mode: 'development',
    devtool: 'inline-source-map',
    devServer: {
        contentBase: dist
    }
});