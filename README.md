# DS210 Final Project

For this final project, I used a [dataset of twitch users](https://arxiv.org/abs/2005.07959) to answer the prompt:

- How often are friends of my friends my friends? This is very generic question, but can you find two vertices who are friends with most similar or most dissimilar sets of connections? What is the right measure of similarity?

## Data

There are six features that can used to see the similarity between the friends of two vertices:

- views (int)
- mature (bool)
- life_time (int)
- dead_account (bool)
- language (str)
- affiliate (bool)

## Calculating Similarity and Dissimilarity

The similarity and dissimilarity is calculated using the following formula:

- Ordinal Similarity: $s = 1 - \frac{||p-q||}{p+q}$
