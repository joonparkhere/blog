import React from "react";
import PropTypes from "prop-types";
import styled from "styled-components";
import Helmet from "react-helmet";

import Section from "Components/Secton"
import Loader from "Components/Loader";
import Message from "Components/Mesage";
import Poster from "Components/Poster";

const Container = styled.div`
    padding: 20px;
`;

const TVPresenter = ({ topRated, popular, airingToday, error, loading }) =>
    <>
        <Helmet>
            <title>TV Shows | Nomflix</title>
        </Helmet>
        {loading ? (
            <Loader />
        ) : (
            <Container>
                {topRated && topRated.length > 0 && (
                    <Section title="Top Rated">
                        {topRated.map(tv => (
                            <Poster
                                key={tv.id}
                                id={tv.id}
                                title={tv.original_name}
                                imageUrl={tv.poster_path}
                                rating={tv.vote_average}
                                year={tv.first_air_date && tv.first_air_date.substring(0, 4)}
                                isMovie={false}
                            />
                        ))}
                    </Section>
                )}
                {popular && popular.length > 0 && (
                    <Section title="Popular">
                        {popular.map(tv => (
                            <Poster
                                key={tv.id}
                                id={tv.id}
                                title={tv.original_name}
                                imageUrl={tv.poster_path}
                                rating={tv.vote_average}
                                year={tv.first_air_date && tv.first_air_date.substring(0, 4)}
                                isMovie={false}
                            />
                        ))}
                    </Section>
                )}
                {airingToday && airingToday.length > 0 && (
                    <Section title="Airing Todat">
                        {airingToday.map(tv => (
                            <Poster
                                key={tv.id}
                                id={tv.id}
                                title={tv.original_name}
                                imageUrl={tv.poster_path}
                                rating={tv.vote_average}
                                year={tv.first_air_date && tv.first_air_date.substring(0, 4)}
                                isMovie={false}
                            />
                        ))}
                    </Section>
                )}
                {error && (
                    <Message text={error} color="#e74c3c"></Message>
                )}
            </Container>
        )};
    </>

TVPresenter.propTypes = {
    topRated: PropTypes.array,
    popular: PropTypes.array,
    airingToday: PropTypes.array,
    error: PropTypes.string,
    loading: PropTypes.bool.isRequired
};

export default TVPresenter;