{% set dimensions = [2, 3, 4] %}
{% for size in dimensions %}
pub trait Vec{{ size }}Swizzles: Sized + Copy + Clone {
{% if size != 2 %}
    type Vec2;
{% endif %}
{% if size != 3 %}
    type Vec3;
{% endif %}
{% if size != 4 %}
    type Vec4;
{% endif %}

{% if size == 4 %}
    #[inline]
    fn xyzw(self) -> Self {
        self
    }
{% elif size == 3 %}
    #[inline]
    fn xyz(self) -> Self {
        self
    }
{% elif size == 2 %}
    #[inline]
    fn xy(self) -> Self {
        self
    }
{% endif %}
}
{% endfor %}
