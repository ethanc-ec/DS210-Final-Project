# DS210 Final Project

For this final project, I will be using a [dataset of twitch users](https://arxiv.org/abs/2005.07959) to answer the prompt:

- How often are friends of my friends my friends? This is very generic question, but can you find two vertices who are friends with most similar or most dissimilar sets of connections? What is the right measure of similarity?

The measures of similarity and dissimilarity is taken from [Penn States' STAT 508](https://online.stat.psu.edu/stat508/lesson/1b/1b.2/1b.2.1). And the [Jaccard Index](https://www.statisticshowto.com/jaccard-index/) is used to measure the similarity for languages.

## Data

There are six features that I used to see the similarity between the friends of two vertices:

- views (int)
- mature (bool)
- life_time (int)
- dead_account (bool)
- language (str)
- affiliate (bool)

## Calculating Similarity and Dissimilarity

The similarity and dissimilarity is calculated using the following formulas:
- Ordinal Similarity: $s = 1 - \frac{||p-q||}{n-1}$
- Jaccard Index: $J(A,B) = \frac{|A \cap B|}{|A \cup B|}$

## Application of Formulas

Ordinal Similarity of Averages:

- mature
- dead_account
- affiliate

Ordinal Similarity of Median:

- views
- life_time

Jaccard Index:

- language
