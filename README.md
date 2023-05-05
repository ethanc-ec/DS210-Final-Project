# DS210 Final Project

For this final project, I used a [dataset of Twitch users](https://arxiv.org/abs/2005.07959) with 168114 nodes to answer the prompt:

- How often are friends of my friends my friends? This is very generic question, but can you find two nodes who are friends with most similar or most dissimilar sets of connections? What is the right measure of similarity?

## Data

There are six features that can used to see the similarity between the friends of two nodes:

- views (int)
- mature (bool)
- life_time (int)
- dead_account (bool)
- language (str)
- affiliate (bool)

However, only views will be used as I presume it is affected by the other features which in turn helps it provide a better view of similarities. For example, to be an affiliate on Twitch, you need to: reach 50 followers, stream for 8 hours, stream on 7 different days, and have an average of 3 viewers. Additionally, Twitch will close accounts that are inactive, as such affiliates will tend to have more views and not be listed as a `dead_account`.

## Calculating Similarity and Dissimilarity

The similarity and dissimilarity is calculated using the following formula:

- Ordinal Similarity: $s = 1 - \frac{||p-q||}{p+q}$

The ordinal calculation will be used to see how similar and dissimilar two nodes as (to my understanding) it "self normalizes" due to the division by $p+q$ which is needed when the data has a large range. Additionally, to compare the two nodes the median amount of views of their connecting nodes will be compared as to prevent heavily influence from outliers. 

## Running The Code

To run, simply clone the project, `cd` into the `twitch_graph` folder, then use `cargo run --release`.
The code will stop looking for pairs of nodes with greater similarity/dissimilarity scores after finding two pairs that each have scores greater than $0.9999$ (or $99.99$% similarity/dissimilarity) to save time and because finding a perfect match (score of 1.0) for both scores will require a full iteration of all node pairs in the worst case.

## Findings

- Max Similarity
  - Score: 1
  - Between nodes 95655 and 150451
  - Notes:
    - The two accounts were created roughly 2.5 years apart. Also, even though their accounts may not be similar (e.g. one is marked as mature, the other is not), it seems that the connections they make are very similar. However, because I am using the median of views, it could be that they both ended up with an odd number of connections and their middle connection ended up being the same person, which lead to a perfect similarity score.
- Max Dissimilarity
  - Score: 0.99995~
  - Between nodes 140894 and 61862
  - Notes: 
    - Node 61862 was created in 2011 while Node 140894 was created in 2018, which could suggest that differences in friends and interests due to what was popular during the time their accounts were created lead to different followings and dissimilarity of their connections.
