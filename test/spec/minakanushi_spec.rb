require 'spec_helper'
require 'yaml'
require 'net/http'
require 'uri'
require 'json'

def api_post(url, payload)
    uri = URI.parse(url)
    req = Net::HTTP::Post.new(uri.request_uri)
    req.body = payload.to_json()
    req['Content-Type'] = 'application/json-rpc'
    
    http = Net::HTTP.new(uri.host, uri.port)
    res = http.request(req)
    data = JSON.parse(res.body)
    
    return data['result']
end

RSpec.describe 'TEST0' do
    it '1 + 1 = 2' do
        expect(1 + 1).to eq(2)
    end
end