/*
 * Transaction Library
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
using OpenAPIDateConverter = Org.OpenAPITools.Client.OpenAPIDateConverter;

namespace Org.OpenAPITools.Model
{
    /// <summary>
    /// A response to the &#x60;ExtractAbiRequest&#x60; containing the ABI and code
    /// </summary>
    [DataContract]
    public partial class ExtractAbiResponse :  IEquatable<ExtractAbiResponse>, IValidatableObject
    {
        /// <summary>
        /// Initializes a new instance of the <see cref="ExtractAbiResponse" /> class.
        /// </summary>
        [JsonConstructorAttribute]
        protected ExtractAbiResponse() { }
        /// <summary>
        /// Initializes a new instance of the <see cref="ExtractAbiResponse" /> class.
        /// </summary>
        /// <param name="code">A hex-encoded string of the package WASM binary (required).</param>
        /// <param name="abi">A hex-encoded string of the SBOR-encoded package ABI (required).</param>
        public ExtractAbiResponse(string code = default(string), string abi = default(string))
        {
            // to ensure "code" is required (not null)
            if (code == null)
            {
                throw new InvalidDataException("code is a required property for ExtractAbiResponse and cannot be null");
            }
            else
            {
                this.Code = code;
            }

            // to ensure "abi" is required (not null)
            if (abi == null)
            {
                throw new InvalidDataException("abi is a required property for ExtractAbiResponse and cannot be null");
            }
            else
            {
                this.Abi = abi;
            }

        }

        /// <summary>
        /// A hex-encoded string of the package WASM binary
        /// </summary>
        /// <value>A hex-encoded string of the package WASM binary</value>
        [DataMember(Name="code", EmitDefaultValue=true)]
        public string Code { get; set; }

        /// <summary>
        /// A hex-encoded string of the SBOR-encoded package ABI
        /// </summary>
        /// <value>A hex-encoded string of the SBOR-encoded package ABI</value>
        [DataMember(Name="abi", EmitDefaultValue=true)]
        public string Abi { get; set; }

        /// <summary>
        /// Returns the string presentation of the object
        /// </summary>
        /// <returns>String presentation of the object</returns>
        public override string ToString()
        {
            var sb = new StringBuilder();
            sb.Append("class ExtractAbiResponse {\n");
            sb.Append("  Code: ").Append(Code).Append("\n");
            sb.Append("  Abi: ").Append(Abi).Append("\n");
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
            return this.Equals(input as ExtractAbiResponse);
        }

        /// <summary>
        /// Returns true if ExtractAbiResponse instances are equal
        /// </summary>
        /// <param name="input">Instance of ExtractAbiResponse to be compared</param>
        /// <returns>Boolean</returns>
        public bool Equals(ExtractAbiResponse input)
        {
            if (input == null)
                return false;

            return 
                (
                    this.Code == input.Code ||
                    (this.Code != null &&
                    this.Code.Equals(input.Code))
                ) && 
                (
                    this.Abi == input.Abi ||
                    (this.Abi != null &&
                    this.Abi.Equals(input.Abi))
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
                if (this.Code != null)
                    hashCode = hashCode * 59 + this.Code.GetHashCode();
                if (this.Abi != null)
                    hashCode = hashCode * 59 + this.Abi.GetHashCode();
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
            yield break;
        }
    }

}
