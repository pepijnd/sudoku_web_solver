const path = require("path");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const MiniCssExtractPlugin = require('mini-css-extract-plugin');
const HtmlWebpackPlugin = require('html-webpack-plugin')
const { CleanWebpackPlugin } = require('clean-webpack-plugin');

const dist = path.resolve(__dirname, "dist");

module.exports = {
    entry: {
        index: "./js/index.js",
    },
    output: {
        path: dist,
        filename: "[contenthash].js"
    },
    module: {
        rules: [
            {
                test: /\.(sa|sc|c)ss$/,
                use: [
                    MiniCssExtractPlugin.loader,
                    {
                        loader: 'css-loader'
                    },
                    {
                        loader: 'postcss-loader',
                        options: {
                            postcssOptions: {
                                plugins: [
                                    "autoprefixer",
                                ]
                            }
                        }
                    },
                    {
                        loader: "sass-loader"
                    }
                ]
            }
        ]
    },
    plugins: [
        new CleanWebpackPlugin(),
        new HtmlWebpackPlugin({
            title: "Sudoku Solver and Analyzer",
            meta: {
                'viewport': 'width=device-width, initial-scale=1, shrink-to-fit=no',
                'color-scheme': 'dark light'
            }
        }),
        new MiniCssExtractPlugin({
            filename: '[contenthash].css',
            chunkFilename: '[chunkhash].css',
          }),
        new WasmPackPlugin({
            crateDirectory: path.resolve(__dirname, "websolver"),
            outDir: path.resolve(__dirname, "pkg"),
            extraArgs: "--no-typescript",
            watchDirectories: [
                path.resolve(__dirname, "websolver/ui"),
                path.resolve(__dirname, "solver"),
            ],
        }),
        new WasmPackPlugin({
            crateDirectory: path.resolve(__dirname, "websolver"),
            outDir: path.resolve(__dirname, "pkg_solver"),
            outName: "worker",
            extraArgs: "--no-typescript -- --no-default-features --features=worker",
            watchDirectories: [
                path.resolve(__dirname, "solver"),
            ],
        }),
    ],
    experiments: {
        asyncWebAssembly: true
    }
};