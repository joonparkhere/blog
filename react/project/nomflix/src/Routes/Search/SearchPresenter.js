import React from "react";
import PropTypes from "prop-types";
import styled from "styled-components";
import Helmet from "react-helmet";

import Loader from "Components/Loader";
import Section from "Components/Secton";
import Message from "Components/Mesage";
import Poster from "Components/Poster";

const Container = styled.div`
    padding: 20px;
`;

const Form = styled.form`
    margin-bottom: 50px;
    width: 100%;
`;

const Input = styled.input`
    all: unset;
    font-size: 28px;
    width: 100%;
`;

const SearchPresenter = ({ movieResults, tvResults, searchTerm, error, loading, handleSubmit, updateTerm }) => (
    <Container>
        <Helmet>
            <title>Search | Nomflix</title>
        </Helmet>
        <Form onSubmit={handleSubmit}>
            <Input placeholder="Search Movies or TV shows..." value={searchTerm} onChange={updateTerm}></Input>
        </Form>
        {loading ? (
            <Loader />
        ) : <>
            {movieResults && movieResults.length > 0 && (
                <Section title="Movie Results">{movieResults.map(movie => (
                    <Poster
                            key={movie.id}
                            id={movie.id}
                            title={movie.original_title}
                            imageUrl={movie.poster_path}
                            rating={movie.vote_average}
                            year={movie.release_date && movie.release_date.substring(0, 4)}
                            isMovie={true}
                        />
                ))}</Section>
            )}
            {tvResults && tvResults.length > 0 && (
                <Section title="TV Show Results">{tvResults.map(tv => (
                    <Poster
                            key={tv.id}
                            id={tv.id}
                            title={tv.original_name}
                            imageUrl={tv.poster_path}
                            rating={tv.vote_average}
                            year={tv.first_air_date && tv.first_air_date.substring(0, 4)}
                            isMovie={true}
                        />
                ))}</Section>
            )}
            {error && (
                <Message text={error} color="#e74c3c"></Message>
            )}
            {movieResults && tvResults && movieResults.length === 0 && tvResults.length === 0 && (
                <Message text="Nothing found" color="#95a6a5"></Message>
            )}
        </>}
    </Container>
);

SearchPresenter.propTypes = {
    movieResults: PropTypes.array,
    tvResults: PropTypes.array,
    searchTerm: PropTypes.string,
    error: PropTypes.string,
    loading: PropTypes.bool.isRequired,
    handleSubmit: PropTypes.func.isRequired,
    updateTerm: PropTypes.func.isRequired
};

export default SearchPresenter;