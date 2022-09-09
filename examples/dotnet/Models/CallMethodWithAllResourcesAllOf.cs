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
    /// CallMethodWithAllResourcesAllOf
    /// </summary>
    [DataContract]
    public partial class CallMethodWithAllResourcesAllOf :  IEquatable<CallMethodWithAllResourcesAllOf>, IValidatableObject
    {
        /// <summary>
        /// Initializes a new instance of the <see cref="CallMethodWithAllResourcesAllOf" /> class.
        /// </summary>
        [JsonConstructorAttribute]
        protected CallMethodWithAllResourcesAllOf() { }
        /// <summary>
        /// Initializes a new instance of the <see cref="CallMethodWithAllResourcesAllOf" /> class.
        /// </summary>
        /// <param name="componentAddress">componentAddress (required).</param>
        /// <param name="methodName">methodName (required).</param>
        public CallMethodWithAllResourcesAllOf(ComponentAddress componentAddress = default(ComponentAddress), String methodName = default(String))
        {
            // to ensure "componentAddress" is required (not null)
            if (componentAddress == null)
            {
                throw new InvalidDataException("componentAddress is a required property for CallMethodWithAllResourcesAllOf and cannot be null");
            }
            else
            {
                this.ComponentAddress = componentAddress;
            }

            // to ensure "methodName" is required (not null)
            if (methodName == null)
            {
                throw new InvalidDataException("methodName is a required property for CallMethodWithAllResourcesAllOf and cannot be null");
            }
            else
            {
                this.MethodName = methodName;
            }

        }

        /// <summary>
        /// Gets or Sets ComponentAddress
        /// </summary>
        [DataMember(Name="component_address", EmitDefaultValue=true)]
        public ComponentAddress ComponentAddress { get; set; }

        /// <summary>
        /// Gets or Sets MethodName
        /// </summary>
        [DataMember(Name="method_name", EmitDefaultValue=true)]
        public String MethodName { get; set; }

        /// <summary>
        /// Returns the string presentation of the object
        /// </summary>
        /// <returns>String presentation of the object</returns>
        public override string ToString()
        {
            var sb = new StringBuilder();
            sb.Append("class CallMethodWithAllResourcesAllOf {\n");
            sb.Append("  ComponentAddress: ").Append(ComponentAddress).Append("\n");
            sb.Append("  MethodName: ").Append(MethodName).Append("\n");
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
            return this.Equals(input as CallMethodWithAllResourcesAllOf);
        }

        /// <summary>
        /// Returns true if CallMethodWithAllResourcesAllOf instances are equal
        /// </summary>
        /// <param name="input">Instance of CallMethodWithAllResourcesAllOf to be compared</param>
        /// <returns>Boolean</returns>
        public bool Equals(CallMethodWithAllResourcesAllOf input)
        {
            if (input == null)
                return false;

            return 
                (
                    this.ComponentAddress == input.ComponentAddress ||
                    (this.ComponentAddress != null &&
                    this.ComponentAddress.Equals(input.ComponentAddress))
                ) && 
                (
                    this.MethodName == input.MethodName ||
                    (this.MethodName != null &&
                    this.MethodName.Equals(input.MethodName))
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
                if (this.ComponentAddress != null)
                    hashCode = hashCode * 59 + this.ComponentAddress.GetHashCode();
                if (this.MethodName != null)
                    hashCode = hashCode * 59 + this.MethodName.GetHashCode();
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
