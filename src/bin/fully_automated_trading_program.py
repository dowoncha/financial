#!/usr/bin/env python
# -*- coding: utf-8 -*-
import numpy
import pandas
import matplotlib.pyplot as plt
import matplotlib.dates as mdates
# Scikit's LinearRegression model
from sklearn.linear_model import LinearRegression
from scipy import stats
import math
import os
import requests
import json
import time
import random
import uuid
from time import localtime, strftime
import datetime
import calendar
import sys
import operator
import sched
from random import shuffle
import holidays
import pytz

set(pytz.all_timezones_set)
YEAR_var = 2018

# ALL EPICS
main_epic_ids = [
    'CS.D.AUDUSD.TODAY.IP',
    'CS.D.EURCHF.TODAY.IP',
    'CS.D.EURGBP.TODAY.IP',
    'CS.D.EURJPY.TODAY.IP',
    'CS.D.EURUSD.TODAY.IP',
    'CS.D.GBPEUR.TODAY.IP',
    'CS.D.GBPUSD.TODAY.IP',
    'CS.D.USDCAD.TODAY.IP',
    'CS.D.USDCHF.TODAY.IP',
    'CS.D.USDJPY.TODAY.IP',
    'CS.D.CADCHF.TODAY.IP',
    'CS.D.CADJPY.TODAY.IP',
    'CS.D.CHFJPY.TODAY.IP',
    'CS.D.EURCAD.TODAY.IP',
    'CS.D.EURSGD.TODAY.IP',
    'CS.D.EURZAR.TODAY.IP',
    'CS.D.GBPCAD.TODAY.IP',
    'CS.D.GBPCHF.TODAY.IP',
    'CS.D.GBPJPY.TODAY.IP',
    'CS.D.GBPSGD.TODAY.IP',
    'CS.D.GBPZAR.TODAY.IP',
    'CS.D.MXNJPY.TODAY.IP',
    'CS.D.NOKJPY.TODAY.IP',
    'CS.D.PLNJPY.TODAY.IP',
    'CS.D.SEKJPY.TODAY.IP',
    'CS.D.SGDJPY.TODAY.IP',
    'CS.D.USDSGD.TODAY.IP',
    'CS.D.USDZAR.TODAY.IP',
    'CS.D.AUDCAD.TODAY.IP',
    'CS.D.AUDCHF.TODAY.IP',
    'CS.D.AUDEUR.TODAY.IP',
    'CS.D.AUDGBP.TODAY.IP',
    'CS.D.AUDJPY.TODAY.IP',
    'CS.D.AUDNZD.TODAY.IP',
    'CS.D.AUDSGD.TODAY.IP',
    'CS.D.EURAUD.TODAY.IP',
    'CS.D.EURNZD.TODAY.IP',
    'CS.D.GBPAUD.TODAY.IP',
    'CS.D.GBPNZD.TODAY.IP',
    'CS.D.NZDAUD.TODAY.IP',
    'CS.D.NZDCAD.TODAY.IP',
    'CS.D.NZDCHF.TODAY.IP',
    'CS.D.NZDEUR.TODAY.IP',
    'CS.D.NZDGBP.TODAY.IP',
    'CS.D.NZDJPY.TODAY.IP',
    'CS.D.NZDUSD.TODAY.IP',
    'CS.D.CHFHUF.TODAY.IP',
    'CS.D.EURCZK.TODAY.IP',
    'CS.D.EURHUF.TODAY.IP',
    'CS.D.EURILS.TODAY.IP',
    'CS.D.EURMXN.TODAY.IP',
    'CS.D.EURPLN.TODAY.IP',
    'CS.D.EURTRY.TODAY.IP',
    'CS.D.GBPCZK.TODAY.IP',
    'CS.D.GBPHUF.TODAY.IP',
    'CS.D.GBPILS.TODAY.IP',
    'CS.D.GBPMXN.TODAY.IP',
    'CS.D.GBPPLN.TODAY.IP',
    'CS.D.GBPTRY.TODAY.IP',
    'CS.D.TRYJPY.TODAY.IP',
    'CS.D.USDCZK.TODAY.IP',
    'CS.D.USDHUF.TODAY.IP',
    'CS.D.USDILS.TODAY.IP',
    'CS.D.USDMXN.TODAY.IP',
    'CS.D.USDPLN.TODAY.IP',
    'CS.D.USDTRY.TODAY.IP',
    'CS.D.CADNOK.TODAY.IP',
    'CS.D.CHFNOK.TODAY.IP',
    'CS.D.EURDKK.TODAY.IP',
    'CS.D.EURNOK.TODAY.IP',
    'CS.D.EURSEK.TODAY.IP',
    'CS.D.GBPDKK.TODAY.IP',
    'CS.D.GBPNOK.TODAY.IP',
    'CS.D.GBPSEK.TODAY.IP',
    'CS.D.NOKSEK.TODAY.IP',
    'CS.D.USDDKK.TODAY.IP',
    'CS.D.USDNOK.TODAY.IP',
    'CS.D.USDSEK.TODAY.IP',
    'CS.D.AUDCNH.TODAY.IP',
    'CS.D.CADCNH.TODAY.IP',
    'CS.D.CNHJPY.TODAY.IP',
    'CS.D.BRLJPY.TODAY.IP',
    'CS.D.GBPINR.TODAY.IP',
    'CS.D.USDBRL.TODAY.IP',
    'CS.D.USDIDR.TODAY.IP',
    'CS.D.USDINR.TODAY.IP',
    'CS.D.USDKRW.TODAY.IP',
    'CS.D.USDMYR.TODAY.IP',
    'CS.D.USDPHP.TODAY.IP',
    'CS.D.USDTWD.TODAY.IP',
    'CS.D.EURCNH.TODAY.IP',
    'CS.D.sp_EURRUB.TODAY.IP',
    'CS.D.GBPCNH.TODAY.IP',
    'CS.D.NZDCNH.TODAY.IP',
    'CS.D.USDCNH.TODAY.IP',
    'CS.D.sp_USDRUB.TODAY.IP']
# ALL EPICS

b_REAL = True #Change this if you want to use your Live/Demo account

if b_REAL:
    REAL_OR_NO_REAL = 'https://api.ig.com/gateway/deal'
    API_ENDPOINT = "https://api.ig.com/gateway/deal/session"
    API_KEY = 'YOUR API KEY HERE'
    data = {"identifier": "YOUR USERNAME HERE", "password": "YOUR PASSWORD HERE"}
else:
    REAL_OR_NO_REAL = 'https://demo-api.ig.com/gateway/deal'
    API_ENDPOINT = "https://demo-api.ig.com/gateway/deal/session"
    API_KEY = 'YOUR API KEY HERE'
    data = {"identifier": "YOUR USERNAME HERE", "password": "YOUR PASSWORD HERE"}

headers = {'Content-Type': 'application/json; charset=utf-8',
           'Accept': 'application/json; charset=utf-8',
           'X-IG-API-KEY': API_KEY,
           'Version': '2'
           }

r = requests.post(API_ENDPOINT, data=json.dumps(data), headers=headers)

headers_json = dict(r.headers)
CST_token = headers_json["CST"]
print(R"CST : " + CST_token)
x_sec_token = headers_json["X-SECURITY-TOKEN"]
print(R"X-SECURITY-TOKEN : " + x_sec_token)

# GET ACCOUNTS
base_url = REAL_OR_NO_REAL + '/accounts'
authenticated_headers = {'Content-Type': 'application/json; charset=utf-8',
                         'Accept': 'application/json; charset=utf-8',
                         'X-IG-API-KEY': API_KEY,
                         'CST': CST_token,
                         'X-SECURITY-TOKEN': x_sec_token}

auth_r = requests.get(base_url, headers=authenticated_headers)
d = json.loads(auth_r.text)

# print ("-----------------DEBUG-----------------")
# print ("#################DEBUG#################")
# print(auth_r.status_code)
# print(auth_r.reason)
# print (auth_r.text)
# print ("-----------------DEBUG-----------------")
# print ("#################DEBUG#################")

for i in d['accounts']:
    if str(i['accountType']) == "SPREADBET":
        print("Spreadbet Account ID is : " + str(i['accountId']))
        spreadbet_acc_id = str(i['accountId'])

# SET SPREAD BET ACCOUNT AS DEFAULT
base_url = REAL_OR_NO_REAL + '/session'
data = {"accountId": spreadbet_acc_id, "defaultAccount": "True"}
auth_r = requests.put(
    base_url,
    data=json.dumps(data),
    headers=authenticated_headers)

##########################################################################
##########################END OF LOGIN CODE###############################
##########################END OF LOGIN CODE###############################
##########################END OF LOGIN CODE###############################
##########################END OF LOGIN CODE###############################
##########################################################################
# PROGRAMMABLE VALUES
orderType_value = "MARKET"
size_value = "5"
expiry_value = "DFB"
guaranteedStop_value = True
currencyCode_value = "GBP"
forceOpen_value = True
stopDistance_value = "15"  # Make this a global variable for ease!

SL_MULTIPLIER = 4
LOW_SL_WATERMARK = 10
HIGH_SL_WATERMARK = 90
STR_CHECK = ""


def is_between(time, time_range):
    if time_range[1] < time_range[0]:
        return time >= time_range[0] or time <= time_range[1]
    return time_range[0] <= time <= time_range[1]


def midpoint(p1, p2):
    return (p1 + p2) / 2


def distance(a, b):
    if (a == b):
        return 0
    elif (a < 0) and (b < 0) or (a > 0) and (b > 0):
        if (a < b):
            return (abs(abs(a) - abs(b)))
        else:
            return -(abs(abs(a) - abs(b)))
    else:
        return math.copysign((abs(a) + abs(b)), b)


def percentage(part, whole):
    return 100 * float(part) / float(whole)


def humanize_time(secs):
    mins, secs = divmod(secs, 60)
    hours, mins = divmod(mins, 60)
    return '%02d:%02d:%02d' % (hours, mins, secs)


def find_good_epics():
    spreads_and_epics = []
    i_count = 0
    pick_from_epics = []
    full_hol_list = []
    ###################################################################
    tz = pytz.timezone('Europe/Berlin')
    todays_date = str(datetime.datetime.now(tz=tz).strftime('%Y-%m-%d'))
    ger_today = str(str("GER_" + str(todays_date)))
    print("Europe/Berlin :- Today's Date is ..." + str(todays_date))
    ###################################################################
    tz = pytz.timezone('Europe/London')
    todays_date = str(datetime.datetime.now(tz=tz).strftime('%Y-%m-%d'))
    gb_today = str(str("GB_" + str(todays_date)))
    print("Europe/London :- Today's Date is ..." + str(todays_date))
    ###################################################################
    tz = pytz.timezone('America/New_York')
    todays_date = str(datetime.datetime.now(tz=tz).strftime('%Y-%m-%d'))
    us_today = str(str("US_" + str(todays_date)))
    print("America/New_York :- Today's Date is ..." + str(todays_date))
    ###################################################################
    tz = pytz.timezone('Australia/Sydney')
    todays_date = str(datetime.datetime.now(tz=tz).strftime('%Y-%m-%d'))
    aus_today = str(str("AUS_" + str(todays_date)))
    print("Australia/Sydney :- Today's Date is ..." + str(todays_date))
    ###################################################################
    tz = pytz.timezone('Asia/Tokyo')
    todays_date = str(datetime.datetime.now(tz=tz).strftime('%Y-%m-%d'))
    jp_today = str(str("JP_" + str(todays_date)))
    print("Asia/Tokyo :- Today's Date is ..." + str(todays_date))
    ###################################################################
    b_ger_hol = False
    b_uk_hol = False
    b_us_hol = False
    b_aus_hol = False
    b_jp_hol = False

    for date, name in sorted(holidays.DE(years=YEAR_var).items()):
        full_hol_list.append(str("GER_" + str(date)))
    for date, name in sorted(holidays.UK(years=YEAR_var).items()):
        full_hol_list.append(str("GB_" + str(date)))
    for date, name in sorted(holidays.US(years=YEAR_var).items()):
        full_hol_list.append(str("US_" + str(date)))
    for date, name in sorted(holidays.AU(years=YEAR_var).items()):
        full_hol_list.append(str("AUS_" + str(date)))
    for date, name in sorted(holidays.JP(years=YEAR_var).items()):
        full_hol_list.append(str("JP_" + str(date)))

    full_hol_list = sorted(full_hol_list)

    for d in full_hol_list:
        #print (d)
        if str(d) == ger_today:
            b_ger_hol = True
        if str(d) == gb_today:
            b_uk_hol = True
        if str(d) == us_today:
            b_us_hol = True
        if str(d) == aus_today:
            b_aus_hol = True
        if str(d) == jp_today:
            b_jp_hol = True

    for epic_id in main_epic_ids:
        tmp_lst = []
        base_url = REAL_OR_NO_REAL + '/markets/' + epic_id
        auth_r = requests.get(base_url, headers=authenticated_headers)
        d = json.loads(auth_r.text)

        try:
            i_count = i_count + 1
            if epic_id.find('MXN') != -1:
                #print("!!DEBUG!!...skipping, FOUND MXN in..." + str(epic_id))
                time.sleep(1)
            elif epic_id.find('SEK') != -1:
                #print("!!DEBUG!!...skipping, FOUND SEK in..." + str(epic_id))
                time.sleep(1)
            elif epic_id.find('NOK') != -1:
                #print("!!DEBUG!!...skipping, FOUND NOK in..." + str(epic_id))
                time.sleep(1)
            elif epic_id.find('CNH') != -1:
                #print("!!DEBUG!!...skipping, FOUND CNH in..." + str(epic_id))
                time.sleep(1)
            else:
                b_TRADE_OK = False
                while True:

                    ###################EUROPE############################
                    ###################EUROPE############################
                    ###################EUROPE############################
                    tz = pytz.timezone('Europe/Berlin')
                    now_time = datetime.datetime.now(tz=tz).strftime('%H:%M')
                    #print ("!!DEBUG!! Europe/Berlin:" + str(now_time))
                    if is_between(str(now_time), ("08:00", "16:00")):
                        #print("!!DEBUG!!...FRANKFURT MARKET OPEN!!")
                        time.sleep(1)
                        STR_CHECK = "EUR"
                        if STR_CHECK in epic_id and b_ger_hol == False:
                            b_TRADE_OK = True
                            break
                    ###################LONDON############################
                    ###################LONDON############################
                    ###################LONDON############################
                    tz = pytz.timezone('Europe/London')
                    now_time = datetime.datetime.now(tz=tz).strftime('%H:%M')
                    while True:
                        if is_between(str(now_time), ("22:00", "22:59")):
                            time.sleep(1)  # Sleeping for the tally up hour
                            print("!!DEBUG!! Tally Up hour:" + str(now_time))
                            now_time = datetime.datetime.now(
                                tz=tz).strftime('%H:%M')
                        else:
                            break
                    #print ("!!DEBUG!! Europe/London:" + str(now_time))
                    if is_between(str(now_time), ("08:00", "16:00")):
                        #print("!!DEBUG!!...LONDON MARKET OPEN!!")
                        time.sleep(1)
                        STR_CHECK = "GBP"
                        if STR_CHECK in epic_id and b_uk_hol == False:
                            b_TRADE_OK = True
                            break
                    ###################NY############################
                    ###################NY############################
                    ###################NY############################
                    tz = pytz.timezone('America/New_York')
                    now_time = datetime.datetime.now(tz=tz).strftime('%H:%M')
                    #print ("!!DEBUG!! America/New_York:" + str(now_time))
                    if is_between(str(now_time), ("08:00", "16:00")):
                        #print("!!DEBUG!!...NEW YORK MARKET OPEN!!")
                        time.sleep(1)
                        STR_CHECK = "USD"
                        if STR_CHECK in epic_id and b_us_hol == False:
                            b_TRADE_OK = True
                            break
                    ###################AUS############################
                    ###################AUS############################
                    ###################AUS############################
                    tz = pytz.timezone('Australia/Sydney')
                    now_time = datetime.datetime.now(tz=tz).strftime('%H:%M')
                    #print ("!!DEBUG!! Australia/Sydney:" + str(now_time))
                    if is_between(str(now_time), ("08:00", "16:00")):
                        #print("!!DEBUG!!...SYDNEY MARKET OPEN!!")
                        time.sleep(1)
                        STR_CHECK = "AUD"
                        if STR_CHECK in epic_id and b_aus_hol == False:
                            b_TRADE_OK = True
                            break
                    ###################TOKYO############################
                    ###################TOKYO############################
                    ###################TOKYO############################
                    tz = pytz.timezone('Asia/Tokyo')
                    now_time = datetime.datetime.now(tz=tz).strftime('%H:%M')
                    #print ("!!DEBUG!! Asia/Tokyo:" + str(now_time))
                    if is_between(str(now_time), ("08:00", "16:00")):
                        #print("!!DEBUG!!...TOKYO MARKET OPEN!!")
                        time.sleep(1)
                        STR_CHECK = "JPY"
                        if STR_CHECK in epic_id and b_jp_hol == False:
                            b_TRADE_OK = True
                            break
                    break

                if b_TRADE_OK:

                    current_bid = d['snapshot']['bid']
                    ask_price = d['snapshot']['offer']
                    spread = float(current_bid) - float(ask_price)
                    if float(spread) >= -1:
                        # tmp_lst.append(epic_id)
                        # spreads_and_epics.append(tmp_lst)
                        pick_from_epics.append(epic_id)
                        # print ("bid : " + str(current_bid))
                        # print ("ask : " + str(ask_price))
                        # print ("-------------------------")
                        # print ("spread : " + str(spread))
                        # print ("-------------------------")
                        print("!!DEBUG!!...FOUND GOOD EPIC..." +
                              str(i_count) + "/" + str(len(main_epic_ids)))
                        time.sleep(1)
                    else:
                        print(
                            "!!DEBUG!!...skipping, NO GOOD EPIC....Checking next epic spreads..." +
                            str(i_count) +
                            "/" +
                            str(
                                len(main_epic_ids)))
                        time.sleep(1)
                        continue

        except Exception as e:
            print(e)
            pass

    return (pick_from_epics)


def are_we_going_to_trade(epic_id, TRADE_DIRECTION, limit):

    if TRADE_DIRECTION == "NONE":
        return None

    limitDistance_value = str(limit)

    ##########################################################################
    print(
        "Order will be a " +
        str(TRADE_DIRECTION) +
        " Order, With a limit of: " +
        str(limitDistance_value))
    print(
        "stopDistance_value for " +
        str(epic_id) +
        " will bet set at " +
        str(stopDistance_value))
    ##########################################################################

    # MAKE AN ORDER
    base_url = REAL_OR_NO_REAL + '/positions/otc'
    data = {
        "direction": TRADE_DIRECTION,
        "epic": epic_id,
        "limitDistance": limitDistance_value,
        "orderType": orderType_value,
        "size": size_value,
        "expiry": expiry_value,
        "guaranteedStop": guaranteedStop_value,
        "currencyCode": currencyCode_value,
        "forceOpen": forceOpen_value,
        "stopDistance": stopDistance_value}
    r = requests.post(
        base_url,
        data=json.dumps(data),
        headers=authenticated_headers)

    print("-----------------DEBUG-----------------")
    print("#################DEBUG#################")
    print(r.status_code)
    print(r.reason)
    print(r.text)
    print("-----------------DEBUG-----------------")
    print("#################DEBUG#################")

    d = json.loads(r.text)
    deal_ref = d['dealReference']
    time.sleep(1)
    # CONFIRM MARKET ORDER
    base_url = REAL_OR_NO_REAL + '/confirms/' + deal_ref
    auth_r = requests.get(base_url, headers=authenticated_headers)
    d = json.loads(auth_r.text)
    DEAL_ID = d['dealId']
    print("DEAL ID : " + str(d['dealId']))
    print(d['dealStatus'])
    print(d['reason'])

    if str(d['reason']) != "SUCCESS":
        print("!!DEBUG!!...!!some thing occurred ERROR!!")
        time.sleep(10)
        print("-----------------DEBUG-----------------")
        print("#################DEBUG#################")
        print("!!DEBUG!! Resuming...")
        print("-----------------DEBUG-----------------")
        print("#################DEBUG#################")
    else:
        print("-----------------DEBUG-----------------")
        print("#################DEBUG#################")
        print("!!DEBUG!!...Yay, ORDER OPEN")
        time.sleep(10)
        print("-----------------DEBUG-----------------")
        print("#################DEBUG#################")


def Chandelier_Exit_formula(TRADE_DIR, ATR, Price):

    if TRADE_DIR == "BUY":

        return float(Price) - float(ATR) * 3

    elif TRADE_DIR == "SELL":

        return float(Price) + float(ATR) * 3


def calculate_stop_loss(d):

    price_ranges = []
    closing_prices = []
    first_time_round_loop = True
    TR_prices = []
    price_compare = "bid"

    for i in d['prices']:
        if first_time_round_loop:
            # First time round loop cannot get previous
            closePrice = i['closePrice'][price_compare]
            closing_prices.append(closePrice)
            high_price = i['highPrice'][price_compare]
            low_price = i['lowPrice'][price_compare]
            price_range = float(high_price - closePrice)
            price_ranges.append(price_range)
            first_time_round_loop = False
        else:
            prev_close = closing_prices[-1]
            ###############################
            closePrice = i['closePrice'][price_compare]
            closing_prices.append(closePrice)
            high_price = i['highPrice'][price_compare]
            low_price = i['lowPrice'][price_compare]
            price_range = float(high_price - closePrice)
            price_ranges.append(price_range)
            TR = max(high_price - low_price,
                     abs(high_price - prev_close),
                     abs(low_price - prev_close))
            TR_prices.append(TR)

    return str(int(float(max(TR_prices))))


if __name__ == '__main__':

    while True:

        try:

            base_url = REAL_OR_NO_REAL + "/accounts"
            auth_r = requests.get(base_url, headers=authenticated_headers)
            d = json.loads(auth_r.text)

            print("-----------------DEBUG-----------------")
            print("#################DEBUG#################")
            print(auth_r.status_code)
            print(auth_r.reason)
            print(auth_r.text)
            print("-----------------DEBUG-----------------")
            print("#################DEBUG#################")

            for i in d['accounts']:
                if str(i['accountType']) == "SPREADBET":
                    balance = i['balance']['balance']
                    deposit = i['balance']['deposit']

            percent_used = percentage(deposit, balance)

            print("-----------------DEBUG-----------------")
            print("#################DEBUG#################")
            print(
                "!!DEBUG!!...Percent of account used ..." +
                str(percent_used))
            print("-----------------DEBUG-----------------")
            print("#################DEBUG#################")

            if float(percent_used) > 70:
                print("!!DEBUG!!...Don't trade, Too much margin used up already")
                time.sleep(60)
                continue
            else:
                print("!!DEBUG!!...OK to trade, Account balance OK!!")

            tradeable_epic_ids = find_good_epics()
            shuffle(tradeable_epic_ids)

        except Exception as e:
            print(e)
            print("!!DEBUG!!...No Suitable Epics...Yet!!, Try again!!")
            continue

        for epic_id in tradeable_epic_ids:

            base_url = REAL_OR_NO_REAL + "/prices/" + epic_id + "/MINUTE_15/16"
            auth_r = requests.get(base_url, headers=authenticated_headers)
            d = json.loads(auth_r.text)

            high_prices = []
            low_prices = []
            mid_prices = []
            close_prices = []
            ATR = calculate_stop_loss(d)

            remaining_allowance = d['allowance']['remainingAllowance']
            reset_time = humanize_time(int(d['allowance']['allowanceExpiry']))
            print("-----------------DEBUG-----------------")
            print("#################DEBUG#################")
            print("Remaining API Calls left: " + str(remaining_allowance))
            print("Time to API Key reset: " + str(reset_time))
            print("-----------------DEBUG-----------------")
            print("#################DEBUG#################")

            for i in d['prices']:

                if i['highPrice']['bid'] is not None:
                    highPrice = i['highPrice']['bid']
                    high_prices.append(highPrice)
                ########################################
                if i['lowPrice']['bid'] is not None:
                    lowPrice = i['lowPrice']['bid']
                    low_prices.append(lowPrice)
                ########################################
                if i['closePrice']['bid'] is not None:
                    closePrice = i['lowPrice']['bid']
                    close_prices.append(closePrice)
                ########################################
                mid_prices.append(float(midpoint(highPrice, lowPrice)))

            close_prices = numpy.asarray(close_prices)
            low_prices = numpy.asarray(low_prices)
            high_prices = numpy.asarray(high_prices)
            mid_prices = numpy.asarray(mid_prices)

            xi = numpy.arange(0, len(low_prices))

            close_prices_slope, close_prices_intercept, close_prices_r_value, close_prices_p_value, close_prices_std_err = stats.linregress(
                xi, close_prices)
            low_prices_slope, low_prices_intercept, low_prices_r_value, low_prices_p_value, low_prices_std_err = stats.linregress(
                xi, low_prices)
            high_prices_slope, high_prices_intercept, high_prices_r_value, high_prices_p_value, high_prices_std_err = stats.linregress(
                xi, high_prices)
            mid_prices_slope, mid_prices_intercept, mid_prices_r_value, mid_prices_p_value, mid_prices_std_err = stats.linregress(
                xi, mid_prices)

            base_url = REAL_OR_NO_REAL + '/markets/' + epic_id
            auth_r = requests.get(
                base_url, headers=authenticated_headers)
            d = json.loads(auth_r.text)

            current_bid = d['snapshot']['bid']

            if distance(current_bid, mid_prices_intercept) > 1:
                TRADE_DIRECTION = "SELL"
                pip_limit = int(abs(float(max(high_prices)) -
                                    float(current_bid)) / SL_MULTIPLIER)
                print("-----------------DEBUG-----------------")
                print("#################DEBUG#################")
                print("!!DEBUG!!...BUY!!")
                print(str(epic_id))
                print(
                    "!!DEBUG!!...Take Profit@...." +
                    str(pip_limit) +
                    " pips")
                print("-----------------DEBUG-----------------")
                print("#################DEBUG#################")
                ce_stop = Chandelier_Exit_formula(
                    TRADE_DIRECTION, ATR, min(low_prices))
                tmp_stop = int(abs(float(current_bid) - (ce_stop)))
            elif distance(current_bid, mid_prices_intercept) < -1:
                TRADE_DIRECTION = "BUY"
                pip_limit = int(abs(float(min(low_prices)) -
                                    float(current_bid)) / SL_MULTIPLIER)
                print("-----------------DEBUG-----------------")
                print("#################DEBUG#################")
                print("!!DEBUG!!...SELL!!")
                print(str(epic_id))
                print(
                    "!!DEBUG!!...Take Profit@...." +
                    str(pip_limit) +
                    " pips")
                print("-----------------DEBUG-----------------")
                print("#################DEBUG#################")
                ce_stop = Chandelier_Exit_formula(
                    TRADE_DIRECTION, ATR, max(high_prices))
                tmp_stop = int(abs(float(current_bid) - (ce_stop)))

            else:
                pip_limit = 9999999
                tmp_stop = "999999"
                TRADE_DIRECTION = "NONE"
                print("!!DEBUG!!...Within trading range...Leave!!")

            if int(pip_limit) <= 1:
                print("!!DEBUG!!...No Trade!!, Pip Limit Under 1")
                TRADE_DIRECTION = "NONE"

            stopDistance_value = str(tmp_stop)

            if int(stopDistance_value) <= LOW_SL_WATERMARK or int(
                    stopDistance_value) >= HIGH_SL_WATERMARK:
                TRADE_DIRECTION = "NONE"

            try:
                are_we_going_to_trade(epic_id, TRADE_DIRECTION, pip_limit)
            except Exception as e:
                print(e)
                print("Something fucked up!!, Try again!!")
                continue