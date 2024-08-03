import sqlite3
import pandas as pd
import plotly.express as px
import plotly.graph_objects as go

# Connect to the SQLite database
conn = sqlite3.connect('sqlite.db')  # replace with your database file
cursor = conn.cursor()

# Query the data
query = "SELECT title, price, seller, shipping, location FROM listings"
df = pd.read_sql_query(query, conn)

# Close the connection
conn.close()

# Filter out rows where price is 0
df = df[df['price'] > 0]

# Calculate the median price
median_price = df['price'].median()

# Add a new column to flag the specific sellers
df['highlight'] = df['seller'].str.contains('tthaco', case=False, na=False)

# Verify the highlight column
print(df[['seller', 'highlight']].head())  # Ensure 'highlight' is correct

# Create a scatter plot using Plotly
fig = px.scatter(df, x='title', y='price', 
                 hover_data=['seller', 'shipping', 'location'],
                 labels={'title': 'Listing Title', 'price': 'Price (GBP)'},
                 title='Price on eBay')

# Update marker properties based on the 'highlight' column
fig.update_traces(
    marker=dict(
        size=df['highlight'].apply(lambda x: 18 if x else 6),  # Size: 18 for highlighted, 6 for others (3x larger)
        color=df['highlight'].apply(lambda x: 'green' if x else 'blue')  # Set color
    )
)

# Add a horizontal line at the median price
fig.add_shape(
    type='line',
    x0=0,
    y0=median_price,
    x1=1,
    y1=median_price,
    xref='paper',
    yref='y',
    line=dict(color='Red', width=2, dash='dash'),
)

# Add annotation for the median price line
fig.add_annotation(
    xref='paper',
    x=1,
    y=median_price,
    xanchor='left',
    yanchor='middle',
    text=f'Median Price: £{median_price:.2f}',
    showarrow=False,
    font=dict(color='Red', size=12)
)

fig.update_layout(xaxis={'categoryorder':'total descending'})  # Sorting the x-axis

# Show the plot
fig.show()

