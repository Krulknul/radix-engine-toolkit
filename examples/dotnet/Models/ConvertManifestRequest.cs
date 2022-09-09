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
    /// ConvertManifestRequest
    /// </summary>
    [DataContract]
    public partial class ConvertManifestRequest :  IEquatable<ConvertManifestRequest>, IValidatableObject
    {
        /// <summary>
        /// Gets or Sets ManifestOutputFormat
        /// </summary>
        [DataMember(Name="manifest_output_format", EmitDefaultValue=true)]
        public ManifestKind ManifestOutputFormat { get; set; }
        /// <summary>
        /// Initializes a new instance of the <see cref="ConvertManifestRequest" /> class.
        /// </summary>
        [JsonConstructorAttribute]
        protected ConvertManifestRequest() { }
        /// <summary>
        /// Initializes a new instance of the <see cref="ConvertManifestRequest" /> class.
        /// </summary>
        /// <param name="transactionVersion">transactionVersion (required).</param>
        /// <param name="networkId">networkId (required).</param>
        /// <param name="manifestOutputFormat">manifestOutputFormat (required).</param>
        /// <param name="manifest">manifest (required).</param>
        public ConvertManifestRequest(int transactionVersion = default(int), int networkId = default(int), ManifestKind manifestOutputFormat = default(ManifestKind), Manifest manifest = default(Manifest))
        {
            // to ensure "transactionVersion" is required (not null)
            if (transactionVersion == null)
            {
                throw new InvalidDataException("transactionVersion is a required property for ConvertManifestRequest and cannot be null");
            }
            else
            {
                this.TransactionVersion = transactionVersion;
            }

            // to ensure "networkId" is required (not null)
            if (networkId == null)
            {
                throw new InvalidDataException("networkId is a required property for ConvertManifestRequest and cannot be null");
            }
            else
            {
                this.NetworkId = networkId;
            }

            // to ensure "manifestOutputFormat" is required (not null)
            if (manifestOutputFormat == null)
            {
                throw new InvalidDataException("manifestOutputFormat is a required property for ConvertManifestRequest and cannot be null");
            }
            else
            {
                this.ManifestOutputFormat = manifestOutputFormat;
            }

            // to ensure "manifest" is required (not null)
            if (manifest == null)
            {
                throw new InvalidDataException("manifest is a required property for ConvertManifestRequest and cannot be null");
            }
            else
            {
                this.Manifest = manifest;
            }

        }

        /// <summary>
        /// Gets or Sets TransactionVersion
        /// </summary>
        [DataMember(Name="transaction_version", EmitDefaultValue=true)]
        public int TransactionVersion { get; set; }

        /// <summary>
        /// Gets or Sets NetworkId
        /// </summary>
        [DataMember(Name="network_id", EmitDefaultValue=true)]
        public int NetworkId { get; set; }


        /// <summary>
        /// Gets or Sets Manifest
        /// </summary>
        [DataMember(Name="manifest", EmitDefaultValue=true)]
        public Manifest Manifest { get; set; }

        /// <summary>
        /// Returns the string presentation of the object
        /// </summary>
        /// <returns>String presentation of the object</returns>
        public override string ToString()
        {
            var sb = new StringBuilder();
            sb.Append("class ConvertManifestRequest {\n");
            sb.Append("  TransactionVersion: ").Append(TransactionVersion).Append("\n");
            sb.Append("  NetworkId: ").Append(NetworkId).Append("\n");
            sb.Append("  ManifestOutputFormat: ").Append(ManifestOutputFormat).Append("\n");
            sb.Append("  Manifest: ").Append(Manifest).Append("\n");
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
            return this.Equals(input as ConvertManifestRequest);
        }

        /// <summary>
        /// Returns true if ConvertManifestRequest instances are equal
        /// </summary>
        /// <param name="input">Instance of ConvertManifestRequest to be compared</param>
        /// <returns>Boolean</returns>
        public bool Equals(ConvertManifestRequest input)
        {
            if (input == null)
                return false;

            return 
                (
                    this.TransactionVersion == input.TransactionVersion ||
                    (this.TransactionVersion != null &&
                    this.TransactionVersion.Equals(input.TransactionVersion))
                ) && 
                (
                    this.NetworkId == input.NetworkId ||
                    (this.NetworkId != null &&
                    this.NetworkId.Equals(input.NetworkId))
                ) && 
                (
                    this.ManifestOutputFormat == input.ManifestOutputFormat ||
                    (this.ManifestOutputFormat != null &&
                    this.ManifestOutputFormat.Equals(input.ManifestOutputFormat))
                ) && 
                (
                    this.Manifest == input.Manifest ||
                    (this.Manifest != null &&
                    this.Manifest.Equals(input.Manifest))
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
                if (this.TransactionVersion != null)
                    hashCode = hashCode * 59 + this.TransactionVersion.GetHashCode();
                if (this.NetworkId != null)
                    hashCode = hashCode * 59 + this.NetworkId.GetHashCode();
                if (this.ManifestOutputFormat != null)
                    hashCode = hashCode * 59 + this.ManifestOutputFormat.GetHashCode();
                if (this.Manifest != null)
                    hashCode = hashCode * 59 + this.Manifest.GetHashCode();
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


            // TransactionVersion (int) maximum
            if(this.TransactionVersion > (int)255)
            {
                yield return new System.ComponentModel.DataAnnotations.ValidationResult("Invalid value for TransactionVersion, must be a value less than or equal to 255.", new [] { "TransactionVersion" });
            }

            // TransactionVersion (int) minimum
            if(this.TransactionVersion < (int)0)
            {
                yield return new System.ComponentModel.DataAnnotations.ValidationResult("Invalid value for TransactionVersion, must be a value greater than or equal to 0.", new [] { "TransactionVersion" });
            }



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
