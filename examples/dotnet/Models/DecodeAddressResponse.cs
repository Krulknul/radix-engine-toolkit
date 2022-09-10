/*
 * Transaction Lib
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 * Generated by: https://github.com/openapitools/openapi-generator.git
 */

using System;
using System.Linq;
using System.IO;
using System.Text;
using System.Text.RegularExpressions;
using System.Collections;
using System.Collections.Generic;
using System.Collections.ObjectModel;
using System.Runtime.Serialization;
using Newtonsoft.Json;
using Newtonsoft.Json.Converters;
using System.ComponentModel.DataAnnotations;

namespace Models
{
    /// <summary>
    /// DecodeAddressResponse
    /// </summary>
    [DataContract]
    public partial class DecodeAddressResponse :  IEquatable<DecodeAddressResponse>, IValidatableObject
    {
        /// <summary>
        /// Gets or Sets EntityType
        /// </summary>
        [DataMember(Name="entity_type", EmitDefaultValue=true)]
        public AddressKind EntityType { get; set; }
        /// <summary>
        /// Initializes a new instance of the <see cref="DecodeAddressResponse" /> class.
        /// </summary>
        [JsonConstructorAttribute]
        protected DecodeAddressResponse() { }
        /// <summary>
        /// Initializes a new instance of the <see cref="DecodeAddressResponse" /> class.
        /// </summary>
        /// <param name="networkId">networkId (required).</param>
        /// <param name="entityType">entityType (required).</param>
        /// <param name="data">data (required).</param>
        /// <param name="hrp">hrp (required).</param>
        /// <param name="address">address (required).</param>
        public DecodeAddressResponse(int networkId = default(int), AddressKind entityType = default(AddressKind), string data = default(string), string hrp = default(string), Address address = default(Address))
        {
            // to ensure "networkId" is required (not null)
            if (networkId == null)
            {
                throw new InvalidDataException("networkId is a required property for DecodeAddressResponse and cannot be null");
            }
            else
            {
                this.NetworkId = networkId;
            }

            // to ensure "entityType" is required (not null)
            if (entityType == null)
            {
                throw new InvalidDataException("entityType is a required property for DecodeAddressResponse and cannot be null");
            }
            else
            {
                this.EntityType = entityType;
            }

            // to ensure "data" is required (not null)
            if (data == null)
            {
                throw new InvalidDataException("data is a required property for DecodeAddressResponse and cannot be null");
            }
            else
            {
                this.Data = data;
            }

            // to ensure "hrp" is required (not null)
            if (hrp == null)
            {
                throw new InvalidDataException("hrp is a required property for DecodeAddressResponse and cannot be null");
            }
            else
            {
                this.Hrp = hrp;
            }

            // to ensure "address" is required (not null)
            if (address == null)
            {
                throw new InvalidDataException("address is a required property for DecodeAddressResponse and cannot be null");
            }
            else
            {
                this.Address = address;
            }

        }

        /// <summary>
        /// Gets or Sets NetworkId
        /// </summary>
        [DataMember(Name="network_id", EmitDefaultValue=true)]
        public int NetworkId { get; set; }


        /// <summary>
        /// Gets or Sets Data
        /// </summary>
        [DataMember(Name="data", EmitDefaultValue=true)]
        public string Data { get; set; }

        /// <summary>
        /// Gets or Sets Hrp
        /// </summary>
        [DataMember(Name="hrp", EmitDefaultValue=true)]
        public string Hrp { get; set; }

        /// <summary>
        /// Gets or Sets Address
        /// </summary>
        [DataMember(Name="address", EmitDefaultValue=true)]
        public Address Address { get; set; }

        /// <summary>
        /// Returns the string presentation of the object
        /// </summary>
        /// <returns>String presentation of the object</returns>
        public override string ToString()
        {
            var sb = new StringBuilder();
            sb.Append("class DecodeAddressResponse {\n");
            sb.Append("  NetworkId: ").Append(NetworkId).Append("\n");
            sb.Append("  EntityType: ").Append(EntityType).Append("\n");
            sb.Append("  Data: ").Append(Data).Append("\n");
            sb.Append("  Hrp: ").Append(Hrp).Append("\n");
            sb.Append("  Address: ").Append(Address).Append("\n");
            sb.Append("}\n");
            return sb.ToString();
        }

        /// <summary>
        /// Returns the JSON string presentation of the object
        /// </summary>
        /// <returns>JSON string presentation of the object</returns>
        public virtual string ToJson()
        {
            return Newtonsoft.Json.JsonConvert.SerializeObject(this, Newtonsoft.Json.Formatting.Indented);
        }

        /// <summary>
        /// Returns true if objects are equal
        /// </summary>
        /// <param name="input">Object to be compared</param>
        /// <returns>Boolean</returns>
        public override bool Equals(object input)
        {
            return this.Equals(input as DecodeAddressResponse);
        }

        /// <summary>
        /// Returns true if DecodeAddressResponse instances are equal
        /// </summary>
        /// <param name="input">Instance of DecodeAddressResponse to be compared</param>
        /// <returns>Boolean</returns>
        public bool Equals(DecodeAddressResponse input)
        {
            if (input == null)
                return false;

            return 
                (
                    this.NetworkId == input.NetworkId ||
                    (this.NetworkId != null &&
                    this.NetworkId.Equals(input.NetworkId))
                ) && 
                (
                    this.EntityType == input.EntityType ||
                    (this.EntityType != null &&
                    this.EntityType.Equals(input.EntityType))
                ) && 
                (
                    this.Data == input.Data ||
                    (this.Data != null &&
                    this.Data.Equals(input.Data))
                ) && 
                (
                    this.Hrp == input.Hrp ||
                    (this.Hrp != null &&
                    this.Hrp.Equals(input.Hrp))
                ) && 
                (
                    this.Address == input.Address ||
                    (this.Address != null &&
                    this.Address.Equals(input.Address))
                );
        }

        /// <summary>
        /// Gets the hash code
        /// </summary>
        /// <returns>Hash code</returns>
        public override int GetHashCode()
        {
            unchecked // Overflow is fine, just wrap
            {
                int hashCode = 41;
                if (this.NetworkId != null)
                    hashCode = hashCode * 59 + this.NetworkId.GetHashCode();
                if (this.EntityType != null)
                    hashCode = hashCode * 59 + this.EntityType.GetHashCode();
                if (this.Data != null)
                    hashCode = hashCode * 59 + this.Data.GetHashCode();
                if (this.Hrp != null)
                    hashCode = hashCode * 59 + this.Hrp.GetHashCode();
                if (this.Address != null)
                    hashCode = hashCode * 59 + this.Address.GetHashCode();
                return hashCode;
            }
        }

        /// <summary>
        /// To validate all properties of the instance
        /// </summary>
        /// <param name="validationContext">Validation context</param>
        /// <returns>Validation Result</returns>
        IEnumerable<System.ComponentModel.DataAnnotations.ValidationResult> IValidatableObject.Validate(ValidationContext validationContext)
        {


            // NetworkId (int) maximum
            if(this.NetworkId > (int)255)
            {
                yield return new System.ComponentModel.DataAnnotations.ValidationResult("Invalid value for NetworkId, must be a value less than or equal to 255.", new [] { "NetworkId" });
            }

            // NetworkId (int) minimum
            if(this.NetworkId < (int)0)
            {
                yield return new System.ComponentModel.DataAnnotations.ValidationResult("Invalid value for NetworkId, must be a value greater than or equal to 0.", new [] { "NetworkId" });
            }

            yield break;
        }
    }

}