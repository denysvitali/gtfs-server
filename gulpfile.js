'use strict';
const gulp = require('gulp');
const pug = require('gulp-pug');
const less = require('gulp-less');
const cssnano = require('gulp-cssnano');
const htmlMinifier = require('gulp-html-minifier');
const imagemin = require('gulp-imagemin');
const del = require('del');
const minifyjs = require('gulp-js-minify');

gulp.task('views', () => {
    return gulp.src('static/src/pug/**/*.pug')
        .pipe(pug({}))
        .pipe(gulp.dest('static/dist/'));
});

gulp.task('html', () => {
    return gulp.src('static/src/pug/**/*.pug')
        .pipe(pug({}))
        .pipe(htmlMinifier({
            removeComments: true,
            collapseWhitespace: true,
            removeTagWhitespace: true
        }))
        .pipe(gulp.dest('static/dist/'));
});

gulp.task('images', () =>
    gulp.src('static/src/img/*')
		.pipe(imagemin())
        .pipe(gulp.dest('static/dist/img/'))
);

gulp.task('less', () => {
    return gulp.src('static/src/less/*.less')
        .pipe(less())
        .pipe(gulp.dest('static/dist/css/'));
});

gulp.task('copy', () => {
    return gulp.src('static/src/dist/**/*')
        .pipe(gulp.dest('static/dist/'));
});

gulp.task('css', () => {
    return gulp.src('static/src/less/*.less')
        .pipe(less())
        .pipe(cssnano())
        .pipe(gulp.dest('static/dist/css'));
});

gulp.task('js', () => {
    return gulp.src('static/src/js/**/*.js')
        .pipe(minifyjs())
        .pipe(gulp.dest('static/dist/js'));
});

gulp.task('clean', (cb) => {
    return del.sync('dist',cb);
});

gulp.task('watch', () => {
    return gulp.watch('static/src/**/*', ['views', 'less', 'copy', 'js']);
});

gulp.task('build', ['views', 'css', 'js', 'images', 'copy']);

gulp.task('cleanandbuild', ['clean', 'build']);

gulp.task('default', ['cleanandbuild']);
