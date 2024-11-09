import { View, Text, StyleSheet, ImageBackground } from 'react-native'
import React from 'react'
import { Link } from 'expo-router'

import backgroundImg from "@/assets/images/background.jpg"

const app = () => {
  return (
    <View style={styles.container}>
      <ImageBackground
        source={backgroundImg}
        resizeMode='cover'
        style={styles.image}
      >
      <Text style={styles.text}>About Us</Text>
      <Text></Text>
      <Text style={styles.textAbout}>Welcome to GameFindr! Ireland's Favourite Video Game Stat Tracking & Visualization App!</Text>
      </ImageBackground>
    </View>
  )
}

export default app

const styles = StyleSheet.create({
  container: {
    flex: 1,
    flexDirection: 'column',
  },

  image: {
    width: '100%',
    height: '100%',
    flex: 1,
    resizeMode: 'cover',
    justifyContent: 'center',
  },

  text: {
    color: 'white',
    fontSize: 42,
    fontWeight: 'bold',
    textAlign: 'center',
    backgroundColor: 'rgba(0,0,0,0.5)',
  },

  textAbout: {
    color: 'white',
    fontSize: 24,
    textAlign: 'center',
    backgroundColor: 'rgba(0,0,0,0.5)',
  }
})